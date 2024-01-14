# About

This project is about aggregating your crypto portfolios from various sources,
like different exchanges, different blockchain wallets and through manual
insertions. Currently only centralized exchanges are supported. To get your
portfolio data for a supported exchange you need to create a configuration file
with your api key and secret key.

Supported exchanges: Binance, Kraken

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

# Example of portfoliosources.toml configuration file

```
[[exchanges]] 
name = "binance" 
api_key = "your_api_key" 
private_key = "your_private_key"

[[exchanges]] 
name = "kraken" 
api_key = "your_api_key" 
private_key = "your_private_key"
```
