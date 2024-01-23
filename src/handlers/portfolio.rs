use axum::{extract::State, Json};

use crate::handlers::common::get_portfolios;
use crate::{
    models::asset_balance::AssetBalance,
    models::portfolio::{Portfolio, Portfolios},
    models::server_state::ServerState,
};
use std::{collections::HashMap, sync::Arc};

pub async fn portfolio(
    State(server_state_arc): State<Arc<ServerState>>,
) -> Json<Portfolio> {
    let portfolios = get_portfolios(&server_state_arc).await;

    aggregate_portfolios(&portfolios).into()
}

fn aggregate_portfolios(portfolios: &Portfolios) -> Portfolio {
    let mut asset_to_amount: HashMap<String, f64> = HashMap::new();
    for portfolio_with_source in portfolios {
        for balance in &portfolio_with_source.portfolio.balances {
            *asset_to_amount.entry(balance.asset.clone()).or_insert(0.0) +=
                balance.amount;
        }
    }

    let balances: Vec<AssetBalance> = asset_to_amount
        .iter()
        .map(|(key, value)| AssetBalance {
            asset: key.clone(),
            amount: *value,
        })
        .collect();

    Portfolio { balances }
}
