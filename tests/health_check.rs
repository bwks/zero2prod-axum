use sqlx::{Connection, PgConnection};
use tokio::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::app;

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app()).await.unwrap();
    });
    format!("{}:{}", address.ip(), address.port())
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://{}/health-check", &address))
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let configuration = get_configuration().expect("failed to read configuration");
    let connection_string = configuration.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("failed to connect to postgres");

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    let response = client
        .post(&format!("http://{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
// Axum returns a 422 error instead of a 400
async fn subscribe_returns_a_422_when_data_is_missing() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("http://{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute request");

        assert_eq!(
            422,
            response.status().as_u16(),
            "The API did no fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
