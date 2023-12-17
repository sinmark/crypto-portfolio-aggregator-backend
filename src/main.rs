mod models;
mod services;

use config::{Config, Environment};
use dotenvy::dotenv;
use serde::Deserialize;
use services::exchange::Exchange;

#[derive(Deserialize)]
struct Configuration {
    pub server_addr: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config_ = Config::builder()
        .add_source(Environment::default())
        .build()
        .unwrap();
    let config: Configuration = config_.try_deserialize().unwrap();
    println!("Server address: {}", config.server_addr);
    let api_key = "";
    let secret_key = "";
    let binance = Exchange::Binance {
        api_key: api_key.to_string(),
        secret_key: secret_key.to_string(),
    };

    match binance.get_account_balance().await {
        Ok(account_info) => println!("{:?}", account_info),
        Err(e) => eprintln!("Error: {}", e),
    }
}
