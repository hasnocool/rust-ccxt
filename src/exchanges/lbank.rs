use crate::{Exchange, CcxtResult, Ticker, OrderBook, OrderBookEntry, Balance, Order, OrderSide, CcxtError};
use async_trait::async_trait;
use reqwest;
use serde_json::Value;
use chrono::Utc;

fn base() -> &'static str { "https://api.lbank.info" }

#[derive(Debug, Default, Clone)]
pub struct LbankExchange { pub api_key: Option<String>, pub secret: Option<String> }
impl LbankExchange { pub fn new() -> Self { Self::default() } pub fn with_auth(k:&str,s:&str)->Self{Self{api_key:Some(k.into()),secret:Some(s.into())}} }

#[async_trait]
impl Exchange for LbankExchange {
    async fn fetch_ticker(&self, symbol:&str) -> CcxtResult<Ticker> {
        let pair = symbol.replace("/", "_").to_lowercase();
        let url = format!("{}/v2/ticker/24hr?symbol={}", base(), pair);
        let resp = reqwest::get(&url).await.map_err(CcxtError::Network)?;
        let data: Value = resp.json().await.map_err(|e| CcxtError::Parse(e.to_string()))?;
        let item = data["data"].as_array().and_then(|a| a.get(0)).cloned().unwrap_or(Value::Null);
        let ticker_data = item.get("ticker").cloned().unwrap_or(Value::Null);
        let high = ticker_data["high"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
        let low = ticker_data["low"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
        let vol = ticker_data["vol"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
        let latest = ticker_data["latest"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
        Ok(Ticker {
            symbol: symbol.into(),
            timestamp: Utc::now().timestamp(),
            high,
            low,
            bid: 0.0,
            ask: 0.0,
            last: latest,
            volume: vol,
            open: None,
            close: None,
        })
    }
    async fn fetch_order_book(&self, symbol:&str, limit:Option<u32>) -> CcxtResult<OrderBook> {
        let pair = symbol.replace("/", "_").to_lowercase();
        let size = limit.unwrap_or(50).to_string();
        let url = format!("{}/v2/depth?symbol={}&size={}", base(), pair, size);
        let resp = reqwest::get(&url).await.map_err(CcxtError::Network)?;
        let data: Value = resp.json().await.map_err(|e| CcxtError::Parse(e.to_string()))?;
        let depth = data["data"].as_object().cloned().unwrap_or_default();
        let bids = depth.get("bids").cloned().unwrap_or(Value::Array(vec![]));
        let asks = depth.get("asks").cloned().unwrap_or(Value::Array(vec![]));
        let parse_depth = |v: &Value| -> Vec<OrderBookEntry> {
            v.as_array().unwrap_or(&vec![]).iter().filter_map(|e| {
                let a = e.as_array()?; 
                if a.len() < 2 { return None; }
                let price = a[0].as_str()?.parse::<f64>().ok()?;
                let amount = a[1].as_str()?.parse::<f64>().ok()?;
                Some(OrderBookEntry { price, amount })
            }).collect()
        };
        let bids_vec = parse_depth(&bids);
        let asks_vec = parse_depth(&asks);
        Ok(OrderBook {
            symbol: symbol.into(),
            timestamp: Utc::now().timestamp(),
            bids: bids_vec,
            asks: asks_vec,
        })
    }
    async fn fetch_balance(&self) -> CcxtResult<Balance> { Err(CcxtError::Unsupported("Lbank auth not implemented".into())) }
    async fn create_order(&self, _s:&str, _side:OrderSide, _amount:f64, _price:Option<f64>) -> CcxtResult<Order> { Err(CcxtError::Unsupported("Lbank auth not implemented".into())) }
    fn id(&self) -> &'static str { "lbank" }
    fn is_implemented(&self) -> bool { true }
}
