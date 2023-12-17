use axum::{extract::State, Json};

use crate::{models::portfolios::Portfolios, services::exchange::Exchange};
use std::sync::Arc;

#[derive(Debug)]
pub struct PortfolioSources {
    pub exchanges: Vec<Exchange>,
}

pub async fn portfolios(
    State(portfolio_sources_arc): State<Arc<PortfolioSources>>,
) -> Json<Portfolios> {
    let portfolio_sources = portfolio_sources_arc.as_ref();
    for exchange in &portfolio_sources.exchanges {
        match exchange.get_portfolio().await {
            Ok(portfolio) => {
                return Portfolios {
                    portfolios: vec![portfolio],
                }
                .into();
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    Portfolios {
        portfolios: Vec::new(),
    }
    .into()
}
