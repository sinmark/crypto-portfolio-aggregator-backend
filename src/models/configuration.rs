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
pub struct PortfolioSourcesConfiguration {
    pub exchanges: Vec<ExchangeConfiguration>,
}
