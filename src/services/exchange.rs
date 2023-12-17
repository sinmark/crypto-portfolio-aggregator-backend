use crate::models::account_balance::AccountBalance;
use crate::services::binance;
use anyhow::Result;

pub enum Exchange {
    Binance { api_key: String, secret_key: String },
}

impl Exchange {
    pub async fn get_account_balance(&self) -> Result<AccountBalance> {
        match self {
            Exchange::Binance {
                api_key,
                secret_key,
            } => binance::get_account_balance(api_key, secret_key).await,
        }
    }
}
