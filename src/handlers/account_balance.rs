use axum::{extract::State, Json};

use crate::{
    models::account_balance::AccountBalance, services::exchange::Exchange,
};
use std::sync::Arc;

#[derive(Debug)]
pub struct PortfolioSources {
    pub exchanges: Vec<Exchange>,
}

pub async fn account_balance(
    State(portfolio_sources_arc): State<Arc<PortfolioSources>>,
) -> Json<AccountBalance> {
    let portfolio_sources = portfolio_sources_arc.as_ref();
    for exchange in &portfolio_sources.exchanges {
        match exchange.get_account_balance().await {
            Ok(account_balance) => {
                return account_balance.into();
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    AccountBalance {
        balances: Vec::new(),
    }
    .into()
}
