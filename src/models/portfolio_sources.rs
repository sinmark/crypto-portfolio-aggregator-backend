use crate::models::blockchain::Blockchains;
use crate::models::exchange::Exchanges;

#[derive(Debug)]
pub struct PortfolioSources {
    pub exchanges: Exchanges,
    pub blockchains: Blockchains,
}
