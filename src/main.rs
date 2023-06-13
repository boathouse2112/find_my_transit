use find_my_transit::config::get_config;
use find_my_transit::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Get TCP listener for our server address
    let config = get_config().expect("Failed to load config file");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;

    // Get database connection
    let connection_string = config.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to create postgres connection pool");

    run(listener, connection_pool)?.await
}
