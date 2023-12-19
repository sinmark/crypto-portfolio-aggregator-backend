use axum::{extract::State, Json};

use crate::{
    models::portfolio_sources::PortfolioSources, models::portfolios::Portfolios,
};
use std::sync::Arc;

pub async fn portfolios(
    State(portfolio_sources_arc): State<Arc<PortfolioSources>>,
) -> Json<Portfolios> {
    let portfolio_sources = portfolio_sources_arc.as_ref();
    let mut result: Portfolios = Vec::new();
    for exchange in &portfolio_sources.exchanges {
        match exchange.get_portfolio().await {
            Ok(portfolio) => result.push(portfolio),
            Err(e) => eprintln!(
                "Error during get_portfolio call: {}, for exchange: {:?}",
                e, exchange
            ),
        }
    }
    result.into()
}
