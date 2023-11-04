use axum::{extract::Path, routing::get, Router};

#[tokio::main]
async fn main() {
    // build the app with a route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/:name", get(move |path| greet(path)));

    axum::Server::bind(&"172.31.255.20:8888".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello {}", &name)
}
