use axum::{extract::State, Json};

use crate::{
    models::portfolio::{Portfolios, Portfolio},
    models::server_state::ServerState,
    models::asset_balance::AssetBalance,
};
use futures::future::join_all;
use std::{sync::Arc, collections::HashMap};
use tokio;

pub async fn portfolio(
    State(server_state_arc): State<Arc<ServerState>>,
) -> Json<Portfolio> {
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
    
    aggregate_portfolios(&portfolios).into()
}

fn aggregate_portfolios(portfolios: &Portfolios) -> Portfolio {
  let mut asset_to_amount: HashMap<String, f64> = HashMap::new();
  for portfolio_with_source in portfolios {
    for balance in &portfolio_with_source.portfolio.balances {
      *asset_to_amount.entry(balance.asset.clone()).or_insert(0.0) += balance.amount;
    }
  }

  let balances: Vec<AssetBalance> = asset_to_amount.iter().map(|(key, value)| AssetBalance {asset: key.clone(), amount: *value}).collect();


  Portfolio { balances }
}