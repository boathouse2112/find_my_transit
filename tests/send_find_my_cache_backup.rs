use std::fs;

use find_my_transit::{config::get_config, FindMyCacheData};
use sqlx::{Connection, PgConnection};

mod utils;

const CACHE_DATA_BACKUP_PATH: &str = "test_resources/find_my_cache_backup.json";

#[tokio::test]
async fn post_find_my_cache_backup_returns_200_for_well_formed_cache_data() -> std::io::Result<()> {
    // Arrange
    let address = utils::spawn_app().await;
    let client = reqwest::Client::new();

    // Get the inner FindMyCacheData out of the array the backup's JSON is wrapped in
    let cache_data_json = fs::read_to_string(CACHE_DATA_BACKUP_PATH)?;
    let cache_data: Vec<FindMyCacheData> = serde_json::from_str(&cache_data_json)?;
    let cache_data_inner = cache_data.get(0);
    let cache_data_json = serde_json::to_string(&cache_data_inner)?;

    let config = get_config().expect("Failed to read config");
    let connection_string = config.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
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

    // Assert
    let status = response.status();

    assert!(status.is_success());

    let saved = sqlx::query!("SELECT id FROM locations")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch failed location snapshot");
    println!("{:?}", saved.id);

    Ok(())
}
