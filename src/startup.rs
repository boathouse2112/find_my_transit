use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;

use crate::routes;

/// Run the find_my_transit server
pub fn run(listener: TcpListener, connection: PgPool) -> std::io::Result<Server> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .service(routes::get_hello)
            .service(routes::post_find_my_cache_backup)
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
