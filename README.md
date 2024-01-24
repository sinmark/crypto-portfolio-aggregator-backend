# About

Aggregates all of your crypto by looking at configured portfolio sources.

Currently supported portfolio sources:

1. Centralized exchanges (Binance, Kraken)
2. Blockchains (Ethereum, Cardano)

# Security

This project is meant strictly for personal use. It a server so you can build a
local frontend for it which uses the data provided by the server. When creating
an API key make sure to only give it reading permissions, as the project only
requires the ability to read the account balance.

# How to use

1. Create portfoliosources.toml file in the root directory which will contain
   configuration for all of your portfolio sources
2. Start the server by running cargo run
3. Use a frontend which targets the server to get the aggregated portfolio (look
   in examples section for supported endpoints)

For Ethereum an API key from [Alchemy](https://www.alchemy.com/) is required. In
the future the project might support more providers.

For Cardano an API key from [Blockfrost](https://blockfrost.io/) is required.

# Examples

## Example of portfoliosources.toml configuration file

```
[[exchanges]] 
name = "binance" 
api_key = "your_api_key" 
private_key = "your_private_key"

[[exchanges]] 
name = "kraken" 
api_key = "your_api_key" 
private_key = "your_private_key"

[[blockchains]]
name = "ethereum"
address = "your_eth_address"
api_key = "your_alchemy_api_key"

[[blockchains]]
name = "cardano"
address = "your_cardano_address"
api_key = "your_blockfrost_api_key"
```

## Example of /portfolio endpoint response

```
{
  "balances": [
    {
      "asset": "ETH",
      "amount": 107686.26657272835
    },
    {
      "asset": "ADA",
      "amount": 2
    }
  ]
}
```

## Example of /portfolios endpoint response

```
[
  {
    "source": "kraken",
    "portfolio": {
      "balances": [
        {
          "asset": "ADA",
          "amount": 2
        }
      ]
    }
  },
  {
    "source": "ethereum",
    "portfolio": {
      "balances": [
        {
          "asset": "ETH",
          "amount": 107686.26657272835
        }
      ]
    }
  }
]
```
