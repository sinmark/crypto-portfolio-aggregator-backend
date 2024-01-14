use crate::models::portfolio::Portfolio;
use anyhow::{Error, Result};

pub async fn get_portfolio(
    _address: &str,
    _api_key: &str,
) -> Result<Portfolio> {
    Err(Error::msg("Unimplemented"))
}
