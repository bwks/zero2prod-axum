use axum::{routing::get, routing::post, Router};
use tokio::net::TcpListener;

use crate::routes::{health_check, subscribe};

pub fn app() -> Router {
    Router::new()
        .route("/health-check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

pub async fn run(listener: TcpListener, app: Router) -> Result<(), std::io::Error> {
    axum::serve(listener, app).await?;
    Ok(())
}
