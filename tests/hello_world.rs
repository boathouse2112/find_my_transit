mod utils;

#[tokio::test]
async fn hello_world() {
    // Arrange
    let app = utils::spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/hello", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!("Hello world!", response.text().await.unwrap());
}
