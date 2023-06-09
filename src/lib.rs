use std::net::TcpListener;

use actix_web::{dev::Server, get, post, web, App, HttpResponse, HttpServer, Responder};
use find_my_cache::FindMyCacheData;

#[allow(dead_code)]
mod find_my_cache;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/find_my_cache_backup")]
async fn post_find_my_cache_backup(cache_data: web::Json<FindMyCacheData>) -> impl Responder {
    println!("{:?}", cache_data);

    HttpResponse::Ok()
}

/// Run the find_my_transit server
pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(post_find_my_cache_backup)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
