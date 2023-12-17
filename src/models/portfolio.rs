use crate::models::asset_balance::AssetBalance;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Portfolio {
    pub source: String,
    pub asset_balances: Vec<AssetBalance>,
}
