use crate::models::{asset_balance::AssetBalance, portfolio::Portfolio};
use anyhow::Result;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::Address,
    utils::format_ether,
};
use url::Url;

const ALCHEMY_BASE_URL: &str = "https://eth-mainnet.g.alchemy.com/v2/";

pub async fn get_portfolio(address_: &str, api_key: &str) -> Result<Portfolio> {
    let url = Url::parse(&format!("{}{}", ALCHEMY_BASE_URL, api_key))?;
    let client = reqwest::Client::new();
    let alchemy = Provider::new(Http::new_with_client(url, client));
    let address: Address = address_.parse()?;
    let balance: f64 =
        format_ether(alchemy.get_balance(address, None).await?).parse()?;

    Ok(Portfolio {
        balances: vec![AssetBalance {
            asset: "ETH".to_string(),
            amount: balance,
        }],
    })
}
