use std::net::TcpListener;

use find_my_transit::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Binding to port 0 gives us a random port to work with
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to random port");
    run(listener)?.await
}
