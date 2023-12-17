use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AssetBalance {
    pub asset: String,
    pub amount: f64,
}
