use crate::models::{asset_balance::AssetBalance, portfolio::Portfolio};
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;

pub async fn get_portfolio(
    address: &str,
    project_id: &str,
) -> Result<Portfolio> {
    let client = Client::new();
    let url = format!("{}{}", BLOCKFROST_BASE_URL, address);

    let res = client
        .get(url)
        .header("project_id", project_id)
        .send()
        .await?;

    let body = res.text().await?;
    let ada_amount = extract_ada_amount(
        serde_json::from_str::<BlockfrostResponse>(&body).map_err(|error| {
            anyhow!(
                "Text that failed to be parsed: {}, the JSON parsing error: {}",
                body,
                error
            )
        })?,
    )?;
    Ok(Portfolio {
        balances: vec![AssetBalance {
            asset: "ADA".to_string(),
            amount: ada_amount,
        }],
    })
}

const BLOCKFROST_BASE_URL: &str =
    "https://cardano-mainnet.blockfrost.io/api/v0/addresses/";
const LOVELACE_DIVISOR: u64 = 1_000_000;

#[derive(Deserialize)]
struct BlockfrostResponse {
    amount: Vec<Amount>,
}

#[derive(Deserialize)]
struct Amount {
    unit: String,
    quantity: String,
}

fn extract_ada_amount(blockfrost_response: BlockfrostResponse) -> Result<f64> {
    blockfrost_response
        .amount
        .iter()
        .try_fold(0.0, |accumulator, amount| {
            let ada = match amount.unit.as_str() {
                "lovelace" => to_ada(amount.quantity.parse::<u64>()?),
                "ada" => amount.quantity.parse::<f64>()?,
                _ => return Err(anyhow!("Unsupported unit!")),
            };
            Ok(accumulator + ada)
        })
}

fn to_ada(lovelace: u64) -> f64 {
    lovelace as f64 / LOVELACE_DIVISOR as f64
}
