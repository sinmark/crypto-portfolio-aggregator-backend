use crate::models::portfolio::Portfolio;
use anyhow::Result;

pub async fn get_portfolio(
    _address_: &str,
    _project_id: &str,
) -> Result<Portfolio> {
    Ok(Portfolio {
        balances: Vec::new(),
    })
}
