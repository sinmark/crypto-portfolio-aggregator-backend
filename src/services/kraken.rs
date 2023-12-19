use crate::models::portfolio::Portfolio;
use anyhow::Result;

pub async fn get_portfolio(
    _public_key: &str,
    _secret_key: &str,
) -> Result<Portfolio> {
    Ok(Portfolio {
        balances: Vec::new(),
    })
}
