use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("172.31.255.20:7777").expect("unable to bind to address");
    run(listener)?.await
}
