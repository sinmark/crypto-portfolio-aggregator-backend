# About

This project is about aggregating your crypto portfolios from various sources,
like different exchanges, different blockchain wallets and through manual
insertions. To get your portfolio data for a supported exchange you need to
create a configuration file with your api key and secret key, different
parameters are required by blockchains.

Supported exchanges: Binance, Kraken

Supported blockchains: Ethereum (Only ETH), Cardano (Only ADA)

# Security

This project is meant strictly for personal use. It is built as a server so that
people can build their own frontends with the data the server offers. As
everything is local how safe it is really depends on the safety of your local
machine. But in any case your assets can't be withdrawn, only seen (if you use
an API key with a small enough permission policy).

# How to use

1. Create portfoliosources.toml file in the root directory which will contain
   configuration for all of your portfolio sources
2. Start the server by running cargo run
3. Use a frontend which targets the server to get the aggregated portfolio
   (currently supported endpoints: /portfolios)

For Ethereum an API key from [Alchemy](https://www.alchemy.com/) is required. In
the future we might support more providers.

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

## Example of /portfolios endpoint response

```
[
   {
      "source":"kraken",
      "portfolio":{
         "balances":[
            {
               "asset":"ADA",
               "amount":2.0
            }
         ]
      }
   },
   {
      "source":"ethereum",
      "portfolio":{
         "balances":[
            {
               "asset":"ETH",
               "amount":107686.26657272835
            }
         ]
      }
   }
]
```
