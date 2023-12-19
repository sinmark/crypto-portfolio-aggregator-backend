use crate::models::portfolio::Portfolio;
use crate::models::portfolios::PortfolioWithSource;
use crate::services::binance;
use crate::services::kraken;
use anyhow::Result;

#[derive(Debug)]
pub enum Exchange {
    Binance {
        api_key: String,
        secret_key: String,
    },
    Kraken {
        public_key: String,
        secret_key: String,
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
                secret_key,
            } => binance::get_portfolio(api_key, secret_key).await.map(
                |portfolio| portfolio.into_portfolio_with_source("binance"),
            ),
            Exchange::Kraken {
                public_key,
                secret_key,
            } => kraken::get_portfolio(public_key, secret_key).await.map(
                |portfolio| portfolio.into_portfolio_with_source("kraken"),
            ),
        }
    }
}
