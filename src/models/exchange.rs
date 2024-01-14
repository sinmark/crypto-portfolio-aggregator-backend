use crate::models::configuration::PortfolioSourcesConfiguration;

#[derive(Debug, Clone)]
pub enum Exchange {
    Binance {
        api_key: String,
        private_key: String,
    },
    Kraken {
        api_key: String,
        private_key: String,
    },
}

pub type Exchanges = Vec<Exchange>;

impl From<&PortfolioSourcesConfiguration> for Exchanges {
    fn from(
        portfolio_sources_configuration: &PortfolioSourcesConfiguration,
    ) -> Exchanges {
        let mut exchanges: Vec<Exchange> = Vec::new();
        for exchange_config in &portfolio_sources_configuration.exchanges {
            match exchange_config.name.as_str() {
                "binance" => exchanges.push(Exchange::Binance {
                    api_key: exchange_config.api_key.clone(),
                    private_key: exchange_config.private_key.clone(),
                }),
                "kraken" => exchanges.push(Exchange::Kraken {
                    api_key: exchange_config.api_key.clone(),
                    private_key: exchange_config.private_key.clone(),
                }),
                _ => {
                    println!("Exchange {} not supported!", exchange_config.name)
                }
            }
        }
        exchanges
    }
}
