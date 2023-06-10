use find_my_transit::config::get_config;
use find_my_transit::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to load config file");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
