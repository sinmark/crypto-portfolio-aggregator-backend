use crate::models::blockchain::Blockchain;
use crate::models::portfolio::PortfolioWithSource;
use crate::services::cardano;
use crate::services::ethereum;
use anyhow::Result;

impl Blockchain {
    pub async fn get_portfolio(&self) -> Result<PortfolioWithSource> {
        match self {
            Blockchain::Ethereum { address, api_key } => {
                ethereum::get_portfolio(address, api_key).await.map(
                    |portfolio| {
                        portfolio.into_portfolio_with_source("ethereum")
                    },
                )
            }
            Blockchain::Cardano {
                address,
                project_id,
            } => cardano::get_portfolio(address, project_id).await.map(
                |portfolio| portfolio.into_portfolio_with_source("cardano"),
            ),
        }
    }
}
