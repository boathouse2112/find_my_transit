use actix_web::{post, web, HttpResponse, Responder};

use crate::FindMyCacheData;

#[post("/find_my_cache_backup")]
pub async fn post_find_my_cache_backup(
    cache_data: web::Json<Vec<FindMyCacheData>>,
) -> impl Responder {
    println!("{:?}", cache_data);

    HttpResponse::Ok()
}
