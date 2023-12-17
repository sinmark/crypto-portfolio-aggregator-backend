use crate::models::asset::Asset;

#[derive(Debug)]
pub struct AccountBalance {
    pub balances: Vec<AssetBalance>,
}

#[derive(Debug)]
pub struct AssetBalance {
    pub asset: Asset,
    pub amount: f64,
}
