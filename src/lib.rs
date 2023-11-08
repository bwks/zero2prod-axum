use axum::{self, extract::Form};
use serde::Deserialize;
use tokio::net::TcpListener;

use axum::{http::StatusCode, response::IntoResponse, routing::get, routing::post, Router};

#[derive(Debug, Deserialize)]
struct FormData {
    email: String,
    name: String,
}

pub fn app() -> Router {
    Router::new()
        .route("/health-check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

pub async fn run(listener: TcpListener, app: Router) -> Result<(), std::io::Error> {
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

async fn subscribe(Form(form_data): Form<FormData>) -> impl IntoResponse {
    // println!("{:#?}", form_data);
    StatusCode::OK
}
