mod handlers;
mod models;
mod services;

use axum::{routing::get, Router};
use config::{Config, Environment};
use dotenvy::dotenv;
use handlers::account_balance::account_balance;
use serde::Deserialize;

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

    let app = Router::new().route("/", get(account_balance));
    let listener = tokio::net::TcpListener::bind(config.server_addr)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
