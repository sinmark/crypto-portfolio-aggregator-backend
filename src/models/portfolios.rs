use crate::models::portfolio::Portfolio;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Portfolios {
    pub portfolios: Vec<Portfolio>,
}
