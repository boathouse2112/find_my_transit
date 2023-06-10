use std::net::TcpListener;

use actix_web::{dev::Server, App, HttpServer};

use crate::routes;

/// Run the find_my_transit server
pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .service(routes::get_hello)
            .service(routes::post_find_my_cache_backup)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
