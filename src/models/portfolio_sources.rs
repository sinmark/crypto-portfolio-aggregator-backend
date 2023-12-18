use crate::services::exchange::Exchange;

#[derive(Debug)]
pub struct PortfolioSources {
    pub exchanges: Vec<Exchange>,
}
