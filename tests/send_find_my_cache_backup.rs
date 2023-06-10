use std::fs;

use find_my_transit::config::get_config;
use sqlx::{Connection, PgConnection};

mod utils;

const CACHE_DATA_BACKUP_PATH: &str = "test_resources/find_my_cache_backup.json";

#[tokio::test]
async fn post_find_my_cache_backup_returns_200_for_well_formed_cache_data() -> std::io::Result<()> {
    // Arrange
    let address = utils::spawn_app();
    let client = reqwest::Client::new();

    let cache_data_json = fs::read_to_string(CACHE_DATA_BACKUP_PATH)?;

    let config = get_config().expect("Failed to read config");
    let connection_string = config.database.connection_string();
    let connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to postgres");

    // Act
    let response = client
        .post(&format!("{}/find_my_cache_backup", address))
        .header("Content-Type", "application/json")
        .body(cache_data_json)
        .send()
        .await
        .expect("Failed to execute request");

    let status = response.status();
    let text = response.text().await.unwrap();

    println!("{:?}", text);

    // Assert
    assert!(status.is_success());

    Ok(())
}
