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

    let timestamp = OffsetDateTime::now_utc();

    let location_id = Uuid::new_v4();
    let address_id = Uuid::new_v4();
    let location_snapshot_id = Uuid::new_v4();

    let query_result = sqlx::query!(
        r#"
        INSERT INTO locations (
            id,
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
        VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        "#,
        location_id,
        location_data.position_type,
        location_data.vertical_accuracy,
        location_data.longitude,
        location_data.floor_level,
        location_data.is_inaccurate,
        location_data.is_old,
        location_data.horizontal_accuracy,
        location_data.latitude,
        timestamp,
        location_data.altitude,
        location_data.location_finished,
    )
    .execute(db_pool.get_ref())
    .await;

    match query_result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
