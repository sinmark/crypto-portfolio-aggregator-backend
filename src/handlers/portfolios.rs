use axum::{extract::State, Json};

use crate::{
    models::portfolio_sources::PortfolioSources, models::portfolios::Portfolios,
};
use std::sync::Arc;

pub async fn portfolios(
    State(portfolio_sources_arc): State<Arc<PortfolioSources>>,
) -> Json<Portfolios> {
    let portfolio_sources = portfolio_sources_arc.as_ref();
    for exchange in &portfolio_sources.exchanges {
        match exchange.get_portfolio().await {
            Ok(portfolio) => {
                return vec![portfolio].into();
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    Vec::new().into()
}
