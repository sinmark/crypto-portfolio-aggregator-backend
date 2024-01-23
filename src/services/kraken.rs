use crate::models::{asset_balance::AssetBalance, portfolio::Portfolio};
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client,
};
use serde::Deserialize;
use sha2::{Digest, Sha256, Sha512};
use std::{collections::HashMap, time::SystemTime, time::UNIX_EPOCH};

pub async fn get_portfolio(
    api_key: &str,
    private_key: &str,
    client: &Client,
) -> Result<Portfolio> {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_millis()
        .to_string();

    let post_data = format!("nonce={}", &nonce);

    let signature =
        kraken_signature(nonce.as_str(), post_data.as_str(), private_key)?;

    let mut request_headers = HeaderMap::new();
    request_headers.append("API-Key", HeaderValue::from_str(api_key)?);
    request_headers.append("API-Sign", HeaderValue::from_str(&signature)?);
    request_headers.append(
        CONTENT_TYPE,
        HeaderValue::from_static(
            "application/x-www-form-urlencoded; charset=utf-8",
        ),
    );

    let url = format!("{}{}", KRAKEN_BASE_URL, URL_PATH);
    let res = client
        .post(url)
        .headers(request_headers)
        .body(post_data)
        .send()
        .await?;

    let body = res.text().await?;
    serde_json::from_str::<KrakenResponse>(&body)
        .map_err(|error| {
            anyhow!(
                "Text that failed to be parsed: {}, the JSON parsing error: {}",
                body,
                error
            )
        })
        .map(Into::into)
}

const KRAKEN_BASE_URL: &str = "https://api.kraken.com";
const URL_PATH: &str = "/0/private/Balance";

fn kraken_signature(
    nonce: &str,
    post_data: &str,
    private_key: &str,
) -> Result<String> {
    let base64_decoded_private_key =
        general_purpose::STANDARD.decode(private_key.as_bytes())?;

    let sha256_input = format!("{}{}", nonce, post_data);
    let mut hasher = Sha256::new();
    hasher.update(sha256_input.as_bytes());
    let digest = hasher.finalize();

    let mut message = Vec::with_capacity(URL_PATH.len() + digest.len());
    message.extend_from_slice(URL_PATH.as_bytes());
    message.extend_from_slice(&digest);

    type HmacSha512 = Hmac<Sha512>;
    let mut hmac = HmacSha512::new_from_slice(&base64_decoded_private_key)?;
    hmac.update(&message);
    let signature_in_bytes = hmac.finalize().into_bytes();

    Ok(general_purpose::STANDARD.encode(signature_in_bytes))
}

#[derive(Deserialize)]
struct KrakenResponse {
    result: HashMap<String, String>,
}

impl From<KrakenResponse> for Portfolio {
    fn from(response_model: KrakenResponse) -> Portfolio {
        Portfolio {
            balances: response_model
                .result
                .into_iter()
                .map(|(asset, amount)| AssetBalance {
                    asset,
                    amount: amount.parse::<f64>().unwrap_or(0.0),
                })
                .collect(),
        }
    }
}
