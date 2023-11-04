use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

#[tokio::main]
async fn main() {
    // build the app with a route
    let app = Router::new().route("/health-check", get(health_check));

    axum::Server::bind(&"172.31.255.20:8888".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
