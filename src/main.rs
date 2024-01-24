mod handlers;
mod models;
mod services;

use crate::models::configuration::{
    PortfolioSourcesConfiguration, ServerConfiguration,
};
use axum::{routing::get, Router};
use config::{Config, Environment, File};
use dotenvy::dotenv;
use handlers::{portfolio::portfolio, portfolios::portfolios};
use models::{
    blockchain::Blockchains, exchange::Exchanges,
    portfolio_sources::PortfolioSources, server_state::ServerState,
};
use reqwest::Client;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let server_config_ = Config::builder()
        .add_source(Environment::default())
        .build()
        .unwrap();
    let server_config: ServerConfiguration = server_config_
        .try_deserialize()
        .unwrap_or(ServerConfiguration {
            server_addr: "localhost:8080".to_string(),
        });
    let portfolio_sources_config_ = Config::builder()
        .add_source(File::with_name("portfoliosources"))
        .build()
        .unwrap();
    let portfolio_sources_config: PortfolioSourcesConfiguration =
        portfolio_sources_config_.try_deserialize().unwrap();

    let exchanges = Exchanges::from(&portfolio_sources_config);

    let blockchains = Blockchains::from(&portfolio_sources_config);

    let client = Client::new();

    let state = Arc::new(ServerState {
        portfolio_sources: PortfolioSources {
            exchanges,
            blockchains,
        },
        client: Arc::new(client),
    });

    let app = Router::new()
        .route("/portfolios", get(portfolios))
        .route("/portfolio", get(portfolio))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(&server_config.server_addr)
        .await
        .unwrap();
    println!("Server running on http://{}.", server_config.server_addr);
    axum::serve(listener, app).await.unwrap();
}
