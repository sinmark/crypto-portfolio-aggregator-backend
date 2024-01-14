use crate::models::exchange::Exchange;

#[derive(Debug)]
pub struct PortfolioSources {
    pub exchanges: Vec<Exchange>,
}
