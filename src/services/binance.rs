use crate::models;
use anyhow::{anyhow, Result};
use hmac_sha256::HMAC;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Deserialize;
use std::{time::SystemTime, time::UNIX_EPOCH};

pub async fn get_portfolio(
    api_key: &str,
    private_key: &str,
) -> Result<models::portfolio::Portfolio> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        .to_string();

    let timestamp_query_parameter = format!("timestamp={}", timestamp);
    let byte_array_signature =
        HMAC::mac(timestamp_query_parameter.as_bytes(), private_key.as_bytes());
    let hex_signature: String = byte_array_signature
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();

    let url = format!(
        "https://testnet.binance.vision/api/v3/account?{}&signature={}",
        timestamp_query_parameter, hex_signature
    );

    let mut request_headers = HeaderMap::new();
    let api_key_header_value = HeaderValue::from_str(api_key)
        .map_err(|e| anyhow!("Header parse error: {}", e))?;
    request_headers.insert("X-MBX-APIKEY", api_key_header_value);
    request_headers
        .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();
    let res = client.get(&url).headers(request_headers).send().await?;
    let body = res.text().await?;
    match serde_json::from_str::<AccountBalance>(&body) {
        Err(error) => Err(anyhow!(
            "Text that failed to be parsed: {}, the JSON parsing error: {}",
            body,
            error
        )),
        Ok(account_balance) => Ok(account_balance),
    }
    .map(Into::into)
}

#[derive(Debug, Deserialize)]
struct AccountBalance {
    balances: Vec<AssetBalance>,
}

#[derive(Debug, Deserialize)]
struct AssetBalance {
    asset: String,
    free: String,
    locked: String,
}

impl From<AccountBalance> for models::portfolio::Portfolio {
    fn from(account_balance: AccountBalance) -> models::portfolio::Portfolio {
        models::portfolio::Portfolio {
            balances: account_balance.balances.iter().map(Into::into).collect(),
        }
    }
}

impl From<&AssetBalance> for models::asset_balance::AssetBalance {
    fn from(
        asset_balance: &AssetBalance,
    ) -> models::asset_balance::AssetBalance {
        models::asset_balance::AssetBalance {
            asset: asset_balance.asset.clone(),
            amount: asset_balance.free.parse::<f64>().unwrap_or(0.0)
                + asset_balance.locked.parse::<f64>().unwrap_or(0.0),
        }
    }
}
