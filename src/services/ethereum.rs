use crate::models::{asset_balance::AssetBalance, portfolio::Portfolio};
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub async fn get_portfolio(
    address: &str,
    api_key: &str,
    client: &Client,
) -> Result<Portfolio> {
    let params = GetBalanceParams(address.to_string(), "latest".to_string());
    let request_body = serde_json::json!({
      "jsonrpc": "2.0",
      "method": "eth_getBalance",
      "params": params,
      "id": 1
    });

    let url = format!("{}{}", ALCHEMY_BASE_URL, api_key);

    let body = client
        .post(&url)
        .json(&request_body)
        .send()
        .await?
        .text()
        .await?;

    let res =
        serde_json::from_str::<GetBalanceResponse>(&body).map_err(|error| {
            anyhow!(
                "Text that failed to be parsed: {}, the JSON parsing error: {}",
                body,
                error
            )
        })?;

    let eth_amount = hex_wei_to_ether(&res.result)?;

    Ok(Portfolio {
        balances: vec![AssetBalance {
            asset: "ETH".to_string(),
            amount: eth_amount,
        }],
    })
}

const ALCHEMY_BASE_URL: &str = "https://eth-mainnet.g.alchemy.com/v2/";

#[derive(Serialize)]
struct GetBalanceParams(String, String);

#[derive(Deserialize)]
struct GetBalanceResponse {
    // ether amount in wei
    result: String,
}

fn hex_wei_to_ether(hex_wei: &str) -> Result<f64> {
    let hex_wei = hex_wei
        .strip_prefix("0x")
        .ok_or(anyhow!("Missing prefix 0x for {}", hex_wei))?;
    let wei_in_bytes = hex::decode(hex_wei)?;

    if wei_in_bytes.len() > 16 {
        return Err(anyhow!("Wrong input hex wei string, the amount should be able to fit inside of u128!"));
    }

    let mut full_bytes = vec![0u8; 16 - wei_in_bytes.len()];
    full_bytes.extend_from_slice(&wei_in_bytes);

    let mut arr = [0u8; 16];
    arr.copy_from_slice(&full_bytes);
    let wei = u128::from_be_bytes(arr);

    let wei_to_ether_conversion_factor: u128 = (10_u128).pow(18);

    let whole_part = wei / wei_to_ether_conversion_factor;

    let fractional_part = wei % wei_to_ether_conversion_factor;

    Ok(whole_part as f64
        + (fractional_part as f64 / wei_to_ether_conversion_factor as f64))
}
