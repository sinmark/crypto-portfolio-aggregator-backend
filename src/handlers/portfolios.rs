use axum::{extract::State, Json};

use crate::{
    models::portfolio::Portfolios,
    models::portfolio_sources::PortfolioSources,
};
use futures::future::join_all;
use std::sync::Arc;
use tokio;

pub async fn portfolios(
    State(portfolio_sources_arc): State<Arc<PortfolioSources>>,
) -> Json<Portfolios> {
    let portfolio_sources = portfolio_sources_arc.as_ref();

    let futures = portfolio_sources.exchanges.iter().map(|exchange| {
        let exchange_cloned = exchange.clone();
        tokio::spawn(async move {
            match exchange_cloned.get_portfolio().await {
                Ok(portfolio) => Some(portfolio),
                Err(e) => {
                    eprintln!(
                "Error during get_portfolio call: {}, for exchange: {:?}",
                e, exchange_cloned 
            );
                    None
                }
            }
        })
    });

    let results = join_all(futures).await;

    let portfolios: Portfolios = results
        .into_iter()
        .filter_map(Result::ok)
        .flatten()
        .collect();

    portfolios.into()
}
