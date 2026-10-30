use crate::{Exchange, CcxtResult, Ticker, OrderBook, OrderBookEntry, Balance, Order, OrderSide, CcxtError};
use async_trait::async_trait;

const BASE: &'static str = "https://api.gateio.ws";

#[derive(Debug, Default, Clone)]
pub struct GateioExchange {
    pub api_key: Option<String>,
    pub secret: Option<String>,
}

impl GateioExchange {
    pub fn new() -> Self {
        Self { api_key: None, secret: None }
    }
    pub fn with_auth(api_key: &str, secret: &str) -> Self {
        Self { api_key: Some(api_key.to_string()), secret: Some(secret.to_string()) }
    }
}

#[async_trait]
impl Exchange for GateioExchange {
    async fn fetch_ticker(&self, symbol: &str) -> CcxtResult<Ticker> {
        let pair = symbol.replace('/', "_");
        let url = format!("{}/v4/spot/tickers?currency_pair={}", BASE, pair);
        let resp = reqwest::get(&url)
            .await
            .map_err(CcxtError::Network)?;
        let data: serde_json::Value = resp.json().await.map_err(|e| CcxtError::Parse(e.to_string()))?;
        let arr = data.as_array().ok_or_else(|| CcxtError::Parse("expected array".into()))?;
        let item = arr.first().ok_or_else(|| CcxtError::Parse("no ticker".into()))?;
        Ok(Ticker {
            symbol: symbol.into(),
            timestamp: chrono::Utc::now().timestamp(),
            high: item["high_24h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            low: item["low_24h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            bid: item["highest_bid"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            ask: item["lowest_ask"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            last: item["last"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            volume: item["base_volume"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            open: None,
            close: None,
        })
    }

    async fn fetch_order_book(&self, symbol: &str, limit: Option<u32>) -> CcxtResult<OrderBook> {
        let pair = symbol.replace('/', "_");
        let lim = limit.unwrap_or(50);
        let url = format!("{}/v4/spot/order_book?currency_pair={}&limit={}", BASE, pair, lim);
        let resp = reqwest::get(&url).await.map_err(CcxtError::Network)?;
        let data: serde_json::Value = resp.json().await.map_err(|e| CcxtError::Parse(e.to_string()))?;
        let bids: Vec<OrderBookEntry> = data["bids"].as_array().unwrap_or(&Vec::new())
            .iter()
            .filter_map(|e| {
                let arr = e.as_array()?;
                Some(OrderBookEntry {
                    price: arr.get(0)?.as_str()?.parse().ok()?,
                    amount: arr.get(1)?.as_str()?.parse().ok()?,
                })
            }).collect();
        let asks: Vec<OrderBookEntry> = data["asks"].as_array().unwrap_or(&Vec::new())
            .iter()
            .filter_map(|e| {
                let arr = e.as_array()?;
                Some(OrderBookEntry {
                    price: arr.get(0)?.as_str()?.parse().ok()?,
                    amount: arr.get(1)?.as_str()?.parse().ok()?,
                })
            }).collect();
        Ok(OrderBook {
            symbol: symbol.into(),
            timestamp: chrono::Utc::now().timestamp(),
            bids,
            asks,
        })
    }

    async fn fetch_balance(&self) -> CcxtResult<Balance> {
        Err(CcxtError::Unsupported("Gate.io balance requires auth".into()))
    }

    async fn create_order(&self, _symbol: &str, _side: OrderSide, _amount: f64, _price: Option<f64>) -> CcxtResult<Order> {
        Err(CcxtError::Unsupported("Gate.io create_order requires auth".into()))
    }

    fn id(&self) -> &'static str {
        "gateio"
    }

    fn is_implemented(&self) -> bool {
        true
    }
}
