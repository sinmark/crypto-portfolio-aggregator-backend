use crate::models::exchange::Exchange;
use crate::models::portfolio::PortfolioWithSource;
use crate::services::binance;
use crate::services::kraken;
use anyhow::Result;

impl Exchange {
    pub async fn get_portfolio(&self) -> Result<PortfolioWithSource> {
        match self {
            Exchange::Binance {
                api_key,
                private_key,
            } => binance::get_portfolio(api_key, private_key).await.map(
                |portfolio| portfolio.into_portfolio_with_source("binance"),
            ),
            Exchange::Kraken {
                api_key,
                private_key,
            } => kraken::get_portfolio(api_key, private_key).await.map(
                |portfolio| portfolio.into_portfolio_with_source("kraken"),
            ),
        }
    }
}
