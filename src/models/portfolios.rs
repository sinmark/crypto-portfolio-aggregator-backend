use crate::models::portfolio::Portfolio;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PortfolioWithSource {
    pub source: String,
    pub portfolio: Portfolio,
}

pub type Portfolios = Vec<PortfolioWithSource>;
