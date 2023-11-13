use tokio::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::{app, run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("failed to read configuration");
    let address = format!(
        "{}:{}",
        configuration.database.host, configuration.application_port
    );

    let listener = TcpListener::bind(address).await.unwrap();
    let app = app();

    // Run forever-ish...
    if let Err(err) = run(listener, app).await {
        eprintln!("server error: {}", err);
    };
    Ok(())
}
