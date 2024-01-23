use axum::{extract::State, Json};

use crate::{
    models::portfolio::Portfolios,
    models::server_state::ServerState,
};
use futures::future::join_all;
use std::sync::Arc;
use tokio;

pub async fn portfolios(
    State(server_state_arc): State<Arc<ServerState>>,
) -> Json<Portfolios> {
    let server_state = server_state_arc.as_ref();
    let portfolio_sources = &server_state.portfolio_sources;
    let client = &server_state.client;

    let exchange_futures = portfolio_sources.exchanges.iter().map(|exchange| {
        let exchange_cloned = exchange.clone();
        let client_arc_cloned = client.clone();

        tokio::spawn(async move {
            match exchange_cloned.get_portfolio(&client_arc_cloned).await {
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
        let client_arc_cloned = client.clone();

        tokio::spawn(async move {
            match blockchain_cloned.get_portfolio(&client_arc_cloned).await {
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
