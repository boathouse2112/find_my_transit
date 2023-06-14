use std::net::TcpListener;

use find_my_transit::config::get_config;
use sqlx::PgPool;

#[derive(Debug)]
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

// Spawn the server, return its URL
pub async fn spawn_app() -> TestApp {
    let config = get_config().expect("Failed to read config");
    // Binding to port 0 gives us a random port to work with
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    // Retrieve the port assigned by the OS
    let port = listener.local_addr().unwrap().port();

    let connection_string = config.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to postgres");

    let server =
        find_my_transit::run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    let address = format!("http://127.0.0.1:{}", port);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}
