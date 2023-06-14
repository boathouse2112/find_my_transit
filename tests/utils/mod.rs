use std::net::TcpListener;

use find_my_transit::config::{get_config, DatabaseSettings};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

#[derive(Debug)]
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

/// Spawn the server, return its URL
pub async fn spawn_app() -> TestApp {
    let mut config = get_config().expect("Failed to read config");
    config.database.database_name = Uuid::new_v4().to_string();

    // Binding to port 0 gives us a random port to work with
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    // Retrieve the port assigned by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let connection_pool = configure_database(&config.database).await;

    let server =
        find_my_transit::run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

/// Create a database, migrate it, and return a connection pool
pub async fn configure_database(settings: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&settings.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, settings.database_name).as_str())
        .await
        .expect("Failed to create database");

    // Migrate database
    let connection_pool = PgPool::connect(&settings.connection_string())
        .await
        .expect("Failed to create connection pool");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");

    connection_pool
}
