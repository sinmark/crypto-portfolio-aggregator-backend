use crate::models::portfolios::PortfolioWithSource;
use crate::services::binance;
use anyhow::Result;

#[derive(Debug)]
pub enum Exchange {
    Binance { api_key: String, secret_key: String },
}

impl Exchange {
    pub async fn get_portfolio(&self) -> Result<PortfolioWithSource> {
        match self {
            Exchange::Binance {
                api_key,
                secret_key,
            } => binance::get_portfolio(api_key, secret_key).await.map(
                |portfolio| PortfolioWithSource {
                    source: "Binance".to_string(),
                    portfolio,
                },
            ),
        }
    }
}
