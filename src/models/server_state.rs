use crate::models::portfolio_sources::PortfolioSources;
use reqwest::Client;
use std::sync::Arc;

pub struct ServerState {
    pub portfolio_sources: PortfolioSources,
    pub client: Arc<Client>,
}
