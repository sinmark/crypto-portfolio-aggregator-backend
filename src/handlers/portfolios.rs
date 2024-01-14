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

    let exchange_futures = portfolio_sources.exchanges.iter().map(|exchange| {
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

    let blockchain_futures = portfolio_sources.blockchains.iter().map(|blockchain| {
        let blockchain_cloned = blockchain.clone();
        tokio::spawn(async move {
            match blockchain_cloned.get_portfolio().await {
                Ok(portfolio) => Some(portfolio),
                Err(e) => {
                    eprintln!(
                "Error during get_portfolio call: {}, for blockchain: {:?}",
                e, blockchain_cloned 
            );
                    None
                }
            }
        })
    });

    let all_futures = exchange_futures.chain(blockchain_futures);

    let results = join_all(all_futures).await;

    let portfolios: Portfolios = results
        .into_iter()
        .filter_map(Result::ok)
        .flatten()
        .collect();

    portfolios.into()
}
