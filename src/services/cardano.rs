use crate::models::{asset_balance::AssetBalance, portfolio::Portfolio};
use anyhow::{anyhow, Result};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde_json::Value;

const BLOCKFROST_BASE_URL: &str =
    "https://cardano-mainnet.blockfrost.io/api/v0/addresses/";

pub async fn get_portfolio(
    address: &str,
    project_id: &str,
) -> Result<Portfolio> {
    let mut request_headers = HeaderMap::new();
    request_headers.append("project_id", HeaderValue::from_str(project_id)?);

    let client = Client::new();
    let url = format!("{}{}", BLOCKFROST_BASE_URL, address);
    let res = client.get(url).headers(request_headers).send().await?;

    let body = res.text().await?;
    let lovelace_amount = match serde_json::from_str::<Value>(&body) {
        Err(error) => Err(anyhow!(
            "Text that failed to be parsed: {}, the JSON parsing error: {}",
            body,
            error
        )),
        Ok(value) => Ok(value),
    }?["amount"][0]["quantity"]
        .as_str()
        .ok_or(anyhow!("Failed to retrieve ADA balance!"))?
        .parse::<u64>()?;
    Ok(Portfolio {
        balances: vec![AssetBalance {
            asset: "ADA".to_string(),
            amount: to_ada(lovelace_amount),
        }],
    })
}

fn to_ada(lovelace: u64) -> f64 {
    let divisor = 1_000_000;
    let ada_whole = (lovelace / divisor) as f64;
    let ada_fractional = (lovelace % divisor) as f64 / divisor as f64;

    ada_whole + ada_fractional
}
