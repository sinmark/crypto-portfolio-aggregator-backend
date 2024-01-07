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

    let mut message = Vec::new();
    message.extend_from_slice(URL_PATH.as_bytes());
    message.extend_from_slice(&digest);

    type HmacSha512 = Hmac<Sha512>;
    let mut hmac =
        HmacSha512::new_from_slice(base64_decoded_private_key.as_slice())?;
    hmac.update(message.as_slice());
    let signature_in_bytes = hmac.finalize().into_bytes();

    Ok(general_purpose::STANDARD.encode(signature_in_bytes))
}
