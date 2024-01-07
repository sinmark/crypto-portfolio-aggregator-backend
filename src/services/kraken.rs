use crate::models::portfolio::Portfolio;
use anyhow::Result;
use base64;
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client,
};
use sha2::{Digest, Sha256, Sha512};
use std::{time::SystemTime, time::UNIX_EPOCH};

const URL_PATH: &str = "/0/private/Balance";

pub async fn get_portfolio(
    api_key: &str,
    private_key: &str,
) -> Result<Portfolio> {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        .to_string();

    let base64_decoded_private_key =
        general_purpose::STANDARD.decode(private_key.as_bytes())?;

    let post_data = format!("nonce={}", &nonce);

    let for_sha_256 = nonce + post_data.as_str();
    let mut hasher = Sha256::new();
    hasher.update(for_sha_256.as_bytes());
    let hashed_value = hasher.finalize();

    let mut message_vec = Vec::new();
    message_vec.extend_from_slice(URL_PATH.as_bytes());
    message_vec.extend_from_slice(&hashed_value);

    type HmacSha512 = Hmac<Sha512>;
    let mut mac =
        HmacSha512::new_from_slice(base64_decoded_private_key.as_slice())?;
    mac.update(message_vec.as_slice());
    let bytes = mac.finalize().into_bytes();

    let signature = general_purpose::STANDARD.encode(bytes);

    let mut request_headers = HeaderMap::new();
    request_headers.append("API-Key", HeaderValue::from_str(api_key)?);
    request_headers.append("API-Sign", HeaderValue::from_str(&signature)?);
    request_headers.append(
        CONTENT_TYPE,
        HeaderValue::from_static(
            "application/x-www-form-urlencoded; charset=utf-8",
        ),
    );

    let client = Client::new();

    let url = format!("https://api.kraken.com{}", URL_PATH);
    let res = client
        .post(url)
        .headers(request_headers)
        .body(post_data)
        .send()
        .await?;

    let body = res.text().await?;
    println!("kraken response body: {}", body);

    Ok(Portfolio {
        balances: Vec::new(),
    })
}
