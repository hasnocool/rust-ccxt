//! CoinMarketCap API (free tier with API key from APIKEYS.md).
//! https://coinmarketcap.com/api/documentation/v1/

use serde_json::Value;

pub struct CoinMarketCap {
    pub api_key: String,
}

impl CoinMarketCap {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self { api_key: api_key.into() }
    }

    /// Latest quotes for symbols.
    pub async fn quotes(&self, symbols: &str) -> Result<Value, reqwest::Error> {
        let url = format!(
            "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol={}",
            symbols
        );
        let client = reqwest::Client::builder().user_agent("rust-ccxt/0.1").build().unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
        let resp = client.get(&url).header("X-CMC_PRO_API_KEY", &self.api_key).send().await?;
        Ok(resp.json().await?)
    }

    /// Top listings.
    pub async fn listings(&self, limit: u32) -> Result<Value, reqwest::Error> {
        let url = format!(
            "https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest?limit={}",
            limit
        );
        let client = reqwest::Client::builder().user_agent("rust-ccxt/0.1").build().unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
        let resp = client.get(&url).header("X-CMC_PRO_API_KEY", &self.api_key).send().await?;
        Ok(resp.json().await?)
    }
}
