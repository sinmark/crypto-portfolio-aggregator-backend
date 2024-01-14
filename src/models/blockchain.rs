use crate::models::configuration::PortfolioSourcesConfiguration;

#[derive(Debug, Clone)]
pub enum Blockchain {
    Ethereum { address: String, api_key: String },
    Cardano { address: String, project_id: String },
}

pub type Blockchains = Vec<Blockchain>;

impl From<&PortfolioSourcesConfiguration> for Blockchains {
    fn from(
        portfolio_sources_configuration: &PortfolioSourcesConfiguration,
    ) -> Blockchains {
        portfolio_sources_configuration
            .blockchains
            .iter()
            .filter_map(|blockchain_config| {
                match blockchain_config.name.as_str() {
                    "ethereum" => Some(Blockchain::Ethereum {
                        address: blockchain_config.address.clone(),
                        api_key: blockchain_config.api_key.clone(),
                    }),
                    "cardano" => Some(Blockchain::Cardano {
                        address: blockchain_config.address.clone(),
                        project_id: blockchain_config.api_key.clone(),
                    }),
                    _ => {
                        eprintln!(
                            "Blockchain {} not supported!",
                            blockchain_config.name
                        );
                        None
                    }
                }
            })
            .collect()
    }
}
