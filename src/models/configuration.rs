use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServerConfiguration {
    pub server_addr: String,
}

#[derive(Deserialize, Debug)]
pub struct ExchangeConfiguration {
    pub name: String,
    pub api_key: String,
    pub private_key: String,
}

#[derive(Deserialize, Debug)]
pub struct BlockchainConfiguration {
    pub name: String,
    pub address: String,
    pub api_key: String,
}

#[derive(Deserialize, Debug)]
pub struct PortfolioSourcesConfiguration {
    pub exchanges: Vec<ExchangeConfiguration>,
    pub blockchains: Vec<BlockchainConfiguration>,
}
