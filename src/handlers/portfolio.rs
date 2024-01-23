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

    aggregate_portfolios(portfolios).into()
}

fn aggregate_portfolios(portfolios: Portfolios) -> Portfolio {
    let asset_to_amount = portfolios
        .into_iter()
        .flat_map(|p| p.portfolio.balances)
        .fold(HashMap::new(), |mut acc, balance| {
            *acc.entry(balance.asset).or_insert(0.0) += balance.amount;
            acc
        });

    let balances = asset_to_amount
        .into_iter()
        .map(|(asset, amount)| AssetBalance { asset, amount })
        .collect();

    Portfolio { balances }
}
