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
        portfolio_sources_configuration
            .exchanges
            .iter()
            .filter_map(|exchange_config| match exchange_config.name.as_str() {
                "binance" => Some(Exchange::Binance {
                    api_key: exchange_config.api_key.clone(),
                    private_key: exchange_config.private_key.clone(),
                }),
                "kraken" => Some(Exchange::Kraken {
                    api_key: exchange_config.api_key.clone(),
                    private_key: exchange_config.private_key.clone(),
                }),
                _ => {
                    eprintln!(
                        "Exchange {} not supported!",
                        exchange_config.name
                    );
                    None
                }
            })
            .collect()
    }
}
