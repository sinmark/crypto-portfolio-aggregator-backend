use crate::models::portfolio::Portfolio;
use anyhow::Result;

pub async fn get_portfolio(
    _api_key: &str,
    _private_key: &str,
) -> Result<Portfolio> {
    Ok(Portfolio {
        balances: Vec::new(),
    })
}
