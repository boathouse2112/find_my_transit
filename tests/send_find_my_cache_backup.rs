use std::fs;

use find_my_transit::FindMyCacheData;
use time::OffsetDateTime;

mod utils;

const CACHE_DATA_BACKUP_PATH: &str = "test_resources/find_my_cache_backup.json";

#[tokio::test]
async fn post_find_my_cache_backup_returns_200_for_well_formed_cache_data() -> std::io::Result<()> {
    // Arrange
    let app = utils::spawn_app().await;
    let client = reqwest::Client::new();

    // Get the inner FindMyCacheData out of the array the backup's JSON is wrapped in
    let cache_data_json = fs::read_to_string(CACHE_DATA_BACKUP_PATH)?;
    let cache_data: Vec<FindMyCacheData> = serde_json::from_str(&cache_data_json)?;
    let cache_data_inner = cache_data.get(0);
    let cache_data_json = serde_json::to_string(&cache_data_inner)?;

    // Act
    let response = client
        .post(&format!("{}/find_my_cache_backup", app.address))
        .header("Content-Type", "application/json")
        .body(cache_data_json)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    let status = response.status();
    assert!(status.is_success());

    // Should have added:
    // - Non-crowd-sourced location
    // - Crowd sourced location
    // - Address
    // - Location snapshot
    let locations = sqlx::query!("SELECT * FROM locations")
        .fetch_all(&app.db_pool)
        .await
        .expect("Failed to select locations");
    let location = sqlx::query!("SELECT * FROM locations WHERE crowd_sourced=false")
        .fetch_all(&app.db_pool)
        .await
        .expect("Failed to select non-crowd-sourced location");
    let crowd_sourced_location = sqlx::query!("SELECT * FROM locations WHERE crowd_sourced=true")
        .fetch_all(&app.db_pool)
        .await
        .expect("Failed to select crowd sourced location");
    let address = sqlx::query!("SELECT * FROM addresses")
        .fetch_all(&app.db_pool)
        .await
        .expect("Failed to select address");
    let location_snapshot = sqlx::query!("SELECT * FROM location_snapshots")
        .fetch_all(&app.db_pool)
        .await
        .expect("Failed to select location snapshots");

    assert_eq!(locations.len(), 2);
    assert_eq!(location.len(), 1);
    assert_eq!(crowd_sourced_location.len(), 1);
    assert_eq!(address.len(), 1);
    assert_eq!(location_snapshot.len(), 1);

    let location = location.get(0).unwrap();
    let crowd_sourced_location = crowd_sourced_location.get(0).unwrap();
    let address = address.get(0).unwrap();
    let location_snapshot = location_snapshot.get(0).unwrap();

    // location fields
    assert_eq!(location.crowd_sourced, false);
    assert_eq!(location.position_type, "ownedDeviceLocation");
    assert_eq!(location.vertical_accuracy, -1);
    assert_eq!(location.longitude, -77.451122697043942);
    assert_eq!(location.is_inaccurate, false);
    assert_eq!(location.is_old, true);
    assert_eq!(location.horizontal_accuracy, 44.338249679264841);
    assert_eq!(location.latitude, 43.086214448400995);
    assert_eq!(
        location.timestamp,
        OffsetDateTime::from_unix_timestamp_nanos(1686289866000 * 1_000_000).unwrap()
    );
    assert_eq!(location.altitude, -1);
    assert_eq!(location.location_finished, true);

    // location crowd-sourced fields
    assert_eq!(crowd_sourced_location.crowd_sourced, true);
    assert_eq!(crowd_sourced_location.position_type, "ownedDeviceLocation");
    assert_eq!(crowd_sourced_location.vertical_accuracy, -1);
    assert_eq!(crowd_sourced_location.longitude, -77.451122697043942);
    assert_eq!(crowd_sourced_location.is_inaccurate, false);
    assert_eq!(crowd_sourced_location.is_old, true);
    assert_eq!(
        crowd_sourced_location.horizontal_accuracy,
        44.338249679264841
    );
    assert_eq!(crowd_sourced_location.latitude, 43.086214448400995);
    assert_eq!(
        crowd_sourced_location.timestamp,
        OffsetDateTime::from_unix_timestamp_nanos(1686289866000 * 1_000_000).unwrap()
    );
    assert_eq!(crowd_sourced_location.altitude, -1);
    assert_eq!(crowd_sourced_location.location_finished, true);

    // address fields
    assert_eq!(address.sub_administrative_area, "モンロー");
    assert_eq!(address.street_address, "35");
    assert_eq!(address.country_code, "US");
    assert_eq!(address.state_code, "NY");
    assert_eq!(address.administrative_area, "ニューヨーク");
    assert_eq!(address.street_name, "Clarkes Crossing");
    assert_eq!(
        address.formatted_address_lines,
        vec![
            "35 Clarkes Crossing",
            "フェアポート, NY  14450",
            "アメリカ合衆国",
        ]
    );
    assert_eq!(
        address.map_item_full_address,
        "35 Clarkes Crossing, フェアポート, NY  14450"
    );
    assert_eq!(address.full_throroughfare, "35 Clarkes Crossing");
    assert_eq!(address.area_of_interest, Vec::<String>::new());
    assert_eq!(address.locality, "フェアポート");
    assert_eq!(address.country, "アメリカ合衆国");

    // location snapshot fields
    assert_eq!(location_snapshot.location_id, location.id);
    assert_eq!(
        location_snapshot.crowd_sourced_location_id,
        crowd_sourced_location.id
    );
    assert_eq!(location_snapshot.address_id, address.id);
    // server_timestamp is generated on the server. We don't know what it should be.

    Ok(())
}
