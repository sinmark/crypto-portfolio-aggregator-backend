use crate::models::asset_balance::AssetBalance;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Portfolio {
    pub balances: Vec<AssetBalance>,
}
