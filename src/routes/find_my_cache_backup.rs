use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::FindMyCacheData;

#[post("/find_my_cache_backup")]
pub async fn post_find_my_cache_backup(
    cache_data: web::Json<FindMyCacheData>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let cache_data = cache_data.0;

    let location_data = cache_data.location;
    let crowd_sourced_location_data = cache_data.crowd_sourced_location;
    let address_data = cache_data.address;

    let server_timestamp = OffsetDateTime::now_utc();

    let location_id = Uuid::new_v4();
    let crowd_sourced_location_id = Uuid::new_v4();
    let address_id = Uuid::new_v4();
    let location_snapshot_id = Uuid::new_v4();

    let location_query_result = sqlx::query!(
        r#"
        INSERT INTO locations (
            id,
            crowd_sourced,
            position_type,
            vertical_accuracy,
            longitude,
            floor_level,
            is_inaccurate,
            is_old,
            horizontal_accuracy,
            latitude,
            timestamp,
            altitude,
            location_finished
        )
        VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        "#,
        location_id,
        false,
        location_data.position_type,
        location_data.vertical_accuracy,
        location_data.longitude,
        location_data.floor_level,
        location_data.is_inaccurate,
        location_data.is_old,
        location_data.horizontal_accuracy,
        location_data.latitude,
        convert_timestamp(location_data.time_stamp),
        location_data.altitude,
        location_data.location_finished,
    )
    .execute(db_pool.get_ref())
    .await;

    let crowd_sourced_location_query_result = sqlx::query!(
        r#"
        INSERT INTO locations (
            id,
            crowd_sourced,
            position_type,
            vertical_accuracy,
            longitude,
            floor_level,
            is_inaccurate,
            is_old,
            horizontal_accuracy,
            latitude,
            timestamp,
            altitude,
            location_finished
        )
        VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        "#,
        crowd_sourced_location_id,
        true,
        crowd_sourced_location_data.position_type,
        crowd_sourced_location_data.vertical_accuracy,
        crowd_sourced_location_data.longitude,
        crowd_sourced_location_data.floor_level,
        crowd_sourced_location_data.is_inaccurate,
        crowd_sourced_location_data.is_old,
        crowd_sourced_location_data.horizontal_accuracy,
        crowd_sourced_location_data.latitude,
        convert_timestamp(crowd_sourced_location_data.time_stamp),
        crowd_sourced_location_data.altitude,
        crowd_sourced_location_data.location_finished,
    )
    .execute(db_pool.get_ref())
    .await;

    let address_query_result = sqlx::query!(
        r#"
        INSERT INTO addresses (
            id,
            sub_administrative_area,
            label,
            street_address,
            country_code,
            state_code,
            administrative_area,
            street_name,
            formatted_address_lines,
            map_item_full_address,
            full_throroughfare,
            area_of_interest,
            locality,
            country
        )
        VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        "#,
        address_id,
        address_data.sub_administrative_area,
        address_data.label,
        address_data.street_address,
        address_data.country_code,
        address_data.state_code,
        &address_data.administrative_area,
        address_data.street_name,
        &address_data.formatted_address_lines,
        address_data.map_item_full_address,
        address_data.full_throroughfare,
        &address_data.area_of_interest,
        address_data.locality,
        address_data.country,
    )
    .execute(db_pool.get_ref())
    .await;

    let location_snapshot_query_result = sqlx::query!(
        r#"
        INSERT INTO location_snapshots (
            id,
            location_id,
            crowd_sourced_location_id,
            address_id,
            server_timestamp
        )
        VALUES($1, $2, $3, $4, $5)
        "#,
        location_snapshot_id,
        location_id,
        crowd_sourced_location_id,
        address_id,
        server_timestamp
    )
    .execute(db_pool.get_ref())
    .await;

    match location_query_result
        .and(crowd_sourced_location_query_result)
        .and(address_query_result)
        .and(location_snapshot_query_result)
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Convert FindMy location timestamps from millisecond-based unix timestamps to `time::OffsetDateTime`
fn convert_timestamp(timestamp_ms: u64) -> OffsetDateTime {
    let timestamp_ns = timestamp_ms * 1_000_000;
    OffsetDateTime::from_unix_timestamp_nanos(timestamp_ns.into()).unwrap()
}
