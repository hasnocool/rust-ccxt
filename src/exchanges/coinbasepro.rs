use crate::{Exchange, CcxtResult, Ticker, OrderBook, OrderBookEntry, Balance, Order, OrderSide, CcxtError};
use async_trait::async_trait;
use reqwest;
use serde_json::Value;
use chrono::Utc;

#[derive(Debug, Default, Clone)]
pub struct CoinbaseproExchange { pub api_key: Option<String>, pub secret: Option<String> }
impl CoinbaseproExchange { pub fn new()->Self{Self::default()} pub fn with_auth(k:&str,s:&str)->Self{Self{api_key:Some(k.into()),secret:Some(s.into())}} fn base()->&'static str{ "https://api.exchange.coinbase.com" } }

#[async_trait]
impl Exchange for CoinbaseproExchange {
    async fn fetch_ticker(&self, symbol:&str)->CcxtResult<Ticker>{
        let pair = symbol.replace("/","-").to_uppercase();
        let url = format!("{}/products/{}/ticker", Self::base(), pair);
        let resp = reqwest::get(&url).await.map_err(CcxtError::Network)?;
        let data: Value = resp.json().await.map_err(|e| CcxtError::Parse(e.to_string()))?;
        Ok(Ticker{ symbol:symbol.into(), timestamp:Utc::now().timestamp(), high:data["high"].as_str().unwrap_or("0").parse().unwrap_or(0.0), low:data["low"].as_str().unwrap_or("0").parse().unwrap_or(0.0), bid:data["bid"].as_str().unwrap_or("0").parse().unwrap_or(0.0), ask:data["ask"].as_str().unwrap_or("0").parse().unwrap_or(0.0), last:data["price"].as_str().unwrap_or("0").parse().unwrap_or(0.0), volume:data["volume"].as_str().unwrap_or("0").parse().unwrap_or(0.0), open:None, close:None })
    }
    async fn fetch_order_book(&self, symbol:&str, limit:Option<u32>)->CcxtResult<OrderBook>{
        let pair = symbol.replace("/","-").to_uppercase();
        let lim = limit.unwrap_or(100);
        let url = format!("{}/products/{}/book?level=2", Self::base(), pair);
        let resp = reqwest::get(&url).await.map_err(CcxtError::Network)?;
        let data: Value = resp.json().await.map_err(|e| CcxtError::Parse(e.to_string()))?;
        let bids:Vec<OrderBookEntry> = data["bids"].as_array().unwrap_or(&vec![]).iter().filter_map(|e| e.as_array().and_then(|a| Some(OrderBookEntry{price:a[0].as_str()?.parse().ok()?, amount:a[1].as_str()?.parse().ok()?}))).collect();
        let asks:Vec<OrderBookEntry> = data["asks"].as_array().unwrap_or(&vec![]).iter().filter_map(|e| e.as_array().and_then(|a| Some(OrderBookEntry{price:a[0].as_str()?.parse().ok()?, amount:a[1].as_str()?.parse().ok()?}))).collect();
        let _lim = lim;
        Ok(OrderBook{symbol:symbol.into(), timestamp:Utc::now().timestamp(), bids, asks})
    }
    async fn fetch_balance(&self)->CcxtResult<Balance>{ Err(CcxtError::Unsupported("Coinbase auth not implemented".into())) }
    async fn create_order(&self,_s:&str,_side:OrderSide,_amount:f64,_price:Option<f64>)->CcxtResult<Order>{ Err(CcxtError::Unsupported("Coinbase auth not implemented".into())) }
    fn id(&self)->&'static str{"coinbasepro"}
    fn is_implemented(&self)->bool{ true }
}
