use tokio::net::TcpListener;

use zero2prod::{app, run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // build the app with a route

    let listener = TcpListener::bind("172.31.255.20:8888").await.unwrap();
    let app = app();

    // Run forever-ish...
    if let Err(err) = run(listener, app).await {
        eprintln!("server error: {}", err);
    };
    Ok(())
}
