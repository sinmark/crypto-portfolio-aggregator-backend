use config::{Config, Environment};
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
struct Configuration {
    pub server_addr: String,
}

fn main() {
    dotenv().ok();
    let config_ = Config::builder()
        .add_source(Environment::default())
        .build()
        .unwrap();
    let config: Configuration = config_.try_deserialize().unwrap();
    println!("Server address: {}", config.server_addr);
}
