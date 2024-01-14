use crate::models::asset_balance::AssetBalance;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Portfolio {
    pub balances: Vec<AssetBalance>,
}

#[derive(Debug, Serialize)]
pub struct PortfolioWithSource {
    pub source: String,
    pub portfolio: Portfolio,
}

pub type Portfolios = Vec<PortfolioWithSource>;

impl Portfolio {
    pub fn into_portfolio_with_source(
        self,
        source: &str,
    ) -> PortfolioWithSource {
        PortfolioWithSource {
            source: source.to_string(),
            portfolio: self,
        }
    }
}
