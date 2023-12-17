use crate::models::portfolio::Portfolio;
use crate::services::binance;
use anyhow::Result;

#[derive(Debug)]
pub enum Exchange {
    Binance { api_key: String, secret_key: String },
}

impl Exchange {
    pub async fn get_portfolio(&self) -> Result<Portfolio> {
        match self {
            Exchange::Binance {
                api_key,
                secret_key,
            } => binance::get_portfolio(api_key, secret_key).await,
        }
    }
}
