use crate::models::{ server_state::ServerState, portfolio::Portfolios};
use futures::future::join_all;

pub async fn get_portfolios(server_state: &ServerState) -> Portfolios {
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

    let mut all_futures: Vec<_> = exchange_futures.collect();
    all_futures.extend(blockchain_futures);

    let results = join_all(all_futures).await;

    results
        .into_iter()
        .filter_map(Result::ok)
        .flatten()
        .collect()
  }