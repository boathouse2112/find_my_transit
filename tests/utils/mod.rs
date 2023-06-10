use std::net::TcpListener;

// Spawn the server, return its URL
pub fn spawn_app() -> String {
    // Binding to port 0 gives us a random port to work with
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    // Retrieve the port assigned by the OS
    let port = listener.local_addr().unwrap().port();

    let server = find_my_transit::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
