use crate::models::portfolio::Portfolio;
use crate::models::portfolios::PortfolioWithSource;
use crate::services::binance;
use crate::services::kraken;
use anyhow::Result;

#[derive(Debug)]
pub enum Exchange {
    Binance {
        api_key: String,
        private_key: String,
    },
    Kraken {
        api_key: String,
        private_key: String,
    },
}

impl Portfolio {
    fn into_portfolio_with_source(self, source: &str) -> PortfolioWithSource {
        PortfolioWithSource {
            source: source.to_string(),
            portfolio: self,
        }
    }
}

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
