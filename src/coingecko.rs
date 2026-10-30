//! CoinGecko API (free tier, no auth for basic endpoints).
//! https://www.coingecko.com/en/api/documentation

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct CoinInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub price_usd: f64,
    pub market_cap: f64,
    pub volume_24h: f64,
    pub price_change_24h: f64,
}

#[derive(Deserialize)]
struct CgPrice {
    usd: Option<f64>,
    #[serde(rename = "usd_market_cap")]
    usd_market_cap: Option<f64>,
    #[serde(rename = "usd_24h_vol")]
    usd_24h_vol: Option<f64>,
    #[serde(rename = "usd_24h_change")]
    usd_24h_change: Option<f64>,
}

pub struct CoinGecko;

impl CoinGecko {
    /// Simple price lookup (free, no key).
    pub async fn price(ids: &str, vs: &str) -> Result<Value, reqwest::Error> {
        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}&include_market_cap=true&include_24hr_vol=true&include_24hr_change=true",
            ids, vs
        );
        let client = reqwest::Client::builder().user_agent("rust-ccxt/0.1").build().unwrap();
        let resp = client.get(&url).send().await?;
        Ok(resp.json().await?)
    }

    /// Top coins by market cap.
    pub async fn markets(vs: &str, per_page: u32, page: u32) -> Result<Vec<CoinInfo>, reqwest::Error> {
        let url = format!(
            "https://api.coingecko.com/api/v3/coins/markets?vs_currency={}&order=market_cap_desc&per_page={}&page={}&sparkline=false",
            vs, per_page, page
        );
        let client = reqwest::Client::builder().user_agent("rust-ccxt/0.1").build().unwrap();
        let resp = client.get(&url).send().await?;
        let arr: Vec<Value> = resp.json().await?;
        Ok(arr.into_iter().map(|v| CoinInfo {
            id: v["id"].as_str().unwrap_or("").to_string(),
            symbol: v["symbol"].as_str().unwrap_or("").to_uppercase(),
            name: v["name"].as_str().unwrap_or("").to_string(),
            price_usd: v["current_price"].as_f64().unwrap_or(0.0),
            market_cap: v["market_cap"].as_f64().unwrap_or(0.0),
            volume_24h: v["total_volume"].as_f64().unwrap_or(0.0),
            price_change_24h: v["price_change_percentage_24h"].as_f64().unwrap_or(0.0),
        }).collect())
    }

    /// Trending coins.
    pub async fn trending() -> Result<Value, reqwest::Error> {
        let url = "https://api.coingecko.com/api/v3/search/trending";
        let client = reqwest::Client::builder().user_agent("rust-ccxt/0.1").build().unwrap();
        let resp = client.get(url).send().await?;
        Ok(resp.json().await?)
    }

    /// Global market data.
    pub async fn global() -> Result<Value, reqwest::Error> {
        let url = "https://api.coingecko.com/api/v3/global";
        let client = reqwest::Client::builder().user_agent("rust-ccxt/0.1").build().unwrap();
        let resp = client.get(url).send().await?;
        Ok(resp.json().await?)
    }
}
