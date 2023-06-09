use std::net::TcpListener;

#[tokio::test]
async fn hello_world() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/hello", address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!("Hello world!", response.text().await.unwrap());
}

// Spawn the server, return its URL
fn spawn_app() -> String {
    // Binding to port 0 gives us a random port to work with
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    // Retrieve the port assigned by the OS
    let port = listener.local_addr().unwrap().port();

    let server = find_my_transit::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
