mod handlers;
mod models;
mod services;

use axum::{routing::get, Router};
use config::{Config, Environment, File};
use dotenvy::dotenv;
use handlers::account_balance::account_balance;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ServerConfiguration {
    pub server_addr: String,
}

#[derive(Deserialize, Debug)]
struct ExchangeConfiguration {
    pub name: String,
    pub api_key: String,
    pub secret_key: String,
}

#[derive(Deserialize, Debug)]
struct PortfolioSourcesConfiguration {
    exchanges: Vec<ExchangeConfiguration>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let server_config_ = Config::builder()
        .add_source(Environment::default())
        .build()
        .unwrap();
    let server_config: ServerConfiguration =
        server_config_.try_deserialize().unwrap();
    let portfolio_sources_config_ = Config::builder()
        .add_source(File::with_name("portfoliosources"))
        .build()
        .unwrap();
    let portfolio_sources_config: PortfolioSourcesConfiguration =
        portfolio_sources_config_.try_deserialize().unwrap();
    println!("What was read: {:?}", portfolio_sources_config.exchanges);

    let app = Router::new().route("/", get(account_balance));
    let listener = tokio::net::TcpListener::bind(server_config.server_addr)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
