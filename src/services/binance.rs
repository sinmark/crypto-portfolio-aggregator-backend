use crate::models;
use anyhow::{anyhow, Result};
use hmac::{Hmac, Mac};
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client,
};
use serde::Deserialize;
use sha2::Sha256;
use std::{time::SystemTime, time::UNIX_EPOCH};

pub async fn get_portfolio(
    api_key: &str,
    private_key: &str,
    client: &Client,
) -> Result<models::portfolio::Portfolio> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_millis()
        .to_string();

    let payload = format!("timestamp={}", timestamp);

    let signature = binance_signature(&payload, private_key)?;

    let url = format!(
        "{}{}?{}&signature={}",
        BINANCE_BASE_URL, URL_PATH, payload, signature
    );

    let mut request_headers = HeaderMap::new();
    request_headers.append("X-MBX-APIKEY", HeaderValue::from_str(api_key)?);
    request_headers
        .append(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let res = client.get(&url).headers(request_headers).send().await?;

    let body = res.text().await?;
    serde_json::from_str::<AccountBalance>(&body)
        .map_err(|error| {
            anyhow!(
                "Text that failed to be parsed: {}, the JSON parsing error: {}",
                body,
                error
            )
        })
        .map(Into::into)
}

const BINANCE_BASE_URL: &str = "https://api.binance.com/";
const URL_PATH: &str = "api/v3/account";

#[derive(Deserialize)]
struct AccountBalance {
    balances: Vec<AssetBalance>,
}

#[derive(Deserialize)]
struct AssetBalance {
    asset: String,
    free: String,
    locked: String,
}

fn binance_signature(payload: &str, private_key: &str) -> Result<String> {
    type HmacSha256 = Hmac<Sha256>;
    let mut hmac = HmacSha256::new_from_slice(private_key.as_bytes())?;
    hmac.update(payload.as_bytes());
    let signature_in_bytes = hmac.finalize().into_bytes();

    Ok(hex::encode(signature_in_bytes))
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
