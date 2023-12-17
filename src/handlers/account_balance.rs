use axum::Json;

use crate::models::account_balance::AccountBalance;

pub async fn account_balance() -> Json<AccountBalance> {
    AccountBalance {
        balances: Vec::new(),
    }
    .into()
}
