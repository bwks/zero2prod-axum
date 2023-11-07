use axum;
use tokio::net::TcpListener;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

pub fn app() -> Router {
    Router::new().route("/health-check", get(health_check))
}

pub async fn run(listener: TcpListener, app: Router) -> Result<(), std::io::Error> {
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
