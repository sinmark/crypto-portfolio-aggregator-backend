use crate::models::account_balance;
use crate::models::asset::Asset;
use anyhow::{anyhow, Result};
use hmac_sha256::HMAC;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Deserialize;
use std::{time::SystemTime, time::UNIX_EPOCH};

pub async fn get_account_balance(
    api_key: &str,
    secret_key: &str,
) -> Result<account_balance::AccountBalance> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        .to_string();

    let query_string = format!("timestamp={}", timestamp);
    let byte_array_signature =
        HMAC::mac(query_string.as_bytes(), secret_key.as_bytes());
    let hex_signature: String = byte_array_signature
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();

    let url = format!(
        "https://testnet.binance.vision/api/v3/account?{}&signature={}",
        query_string, hex_signature
    );

    let mut headers = HeaderMap::new();
    let api_key_header = HeaderValue::from_str(api_key)
        .map_err(|e| anyhow!("Header parse error: {}", e))?;
    headers.insert("X-MBX-APIKEY", api_key_header);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();
    let res = client.get(&url).headers(headers).send().await?;
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

impl From<AccountBalance> for account_balance::AccountBalance {
    fn from(
        account_balance: AccountBalance,
    ) -> account_balance::AccountBalance {
        account_balance::AccountBalance {
            balances: account_balance.balances.iter().map(Into::into).collect(),
        }
    }
}

impl From<&AssetBalance> for account_balance::AssetBalance {
    fn from(asset_balance: &AssetBalance) -> account_balance::AssetBalance {
        account_balance::AssetBalance {
            asset: binance_asset_string_to_standardized_asset_struct(
                &asset_balance.asset,
            ),
            amount: asset_balance.free.parse::<f64>().unwrap_or(0.0)
                + asset_balance.locked.parse::<f64>().unwrap_or(0.0),
        }
    }
}

fn binance_asset_string_to_standardized_asset_struct(asset: &str) -> Asset {
    match asset {
        "ADA" => Asset::Ada,
        "DOT" => Asset::Dot,
        "ETH" => Asset::Eth,
        "BTC" => Asset::Btc,
        "USDT" => Asset::Usdt,
        "BNB" => Asset::Bnb,
        "XRP" => Asset::Xrp,
        "LTC" => Asset::Ltc,
        _ => Asset::Uknown(asset.to_string()),
    }
}