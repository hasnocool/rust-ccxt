use crate::{Exchange, CcxtResult, Ticker, OrderBook, OrderBookEntry, Balance, Order, OrderSide, CcxtError};
use async_trait::async_trait;
use reqwest;
use serde_json::Value;
use chrono::Utc;

fn base() -> &'static str { "https://api.binance.com" }

#[derive(Debug, Default, Clone)]
pub struct BinanceExchange { pub api_key: Option<String>, pub secret: Option<String> }
impl BinanceExchange { pub fn new() -> Self { Self::default() } pub fn with_auth(k: &str, s: &str) -> Self { Self { api_key: Some(k.into()), secret: Some(s.into()) } } }

#[async_trait]
impl Exchange for BinanceExchange {
    async fn fetch_ticker(&self, symbol: &str) -> CcxtResult<Ticker> {
        let url = format!("{}/api/v3/ticker/24hr?symbol={}", base(), symbol);
        let resp = reqwest::get(&url).await.map_err(CcxtError::Network)?;
        let data: Value = resp.json().await.map_err(|e| CcxtError::Parse(e.to_string()))?;
        Ok(Ticker {
            symbol: symbol.into(),
            timestamp: Utc::now().timestamp(),
            high: data["highPrice"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            low: data["lowPrice"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            bid: data["bidPrice"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            ask: data["askPrice"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            last: data["lastPrice"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            volume: data["volume"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            open: Some(data["openPrice"].as_str().unwrap_or("0").parse().unwrap_or(0.0)),
            close: None,
        })
    }
    async fn fetch_order_book(&self, symbol: &str, limit: Option<u32>) -> CcxtResult<OrderBook> {
        let lim = limit.unwrap_or(50);
        let url = format!("{}/api/v3/depth?symbol={}&limit={}", base(), symbol, lim);
        let resp = reqwest::get(&url).await.map_err(CcxtError::Network)?;
        let data: Value = resp.json().await.map_err(|e| CcxtError::Parse(e.to_string()))?;
        let bids: Vec<OrderBookEntry> = data["bids"].as_array().unwrap_or(&vec![]).iter().filter_map(|e| {
            let a = e.as_array()?; if a.len() < 2 { return None; }
            Some(OrderBookEntry { price: a[0].as_str()?.parse().ok()?, amount: a[1].as_str()?.parse().ok()? })
        }).collect();
        let asks: Vec<OrderBookEntry> = data["asks"].as_array().unwrap_or(&vec![]).iter().filter_map(|e| {
            let a = e.as_array()?; if a.len() < 2 { return None; }
            Some(OrderBookEntry { price: a[0].as_str()?.parse().ok()?, amount: a[1].as_str()?.parse().ok()? })
        }).collect();
        Ok(OrderBook { symbol: symbol.into(), timestamp: Utc::now().timestamp(), bids, asks })
    }
    async fn fetch_balance(&self) -> CcxtResult<Balance> { Err(CcxtError::Unsupported("Binance auth not implemented".into())) }
    async fn create_order(&self, _s: &str, _side: OrderSide, _amount: f64, _price: Option<f64>) -> CcxtResult<Order> { Err(CcxtError::Unsupported("Binance auth not implemented".into())) }
    fn id(&self) -> &'static str { "binance" }
    fn is_implemented(&self) -> bool { true }
}
