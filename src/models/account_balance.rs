use serde::Serialize;

use crate::models::asset::Asset;

#[derive(Debug, Serialize)]
pub struct AccountBalance {
    pub balances: Vec<AssetBalance>,
}

#[derive(Debug, Serialize)]
pub struct AssetBalance {
    pub asset: Asset,
    pub amount: f64,
}
