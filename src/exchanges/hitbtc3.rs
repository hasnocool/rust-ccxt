use crate::{Exchange, CcxtResult, Ticker, OrderBook, OrderBookEntry, Balance, Order, OrderSide, CcxtError};
use async_trait::async_trait;
use reqwest;
use serde_json::Value;
use chrono::Utc;

fn base() -> &'static str { "BASE_URL" }

#[derive(Debug, Default, Clone)]
pub struct Hitbtc3Exchange { pub api_key: Option<String>, pub secret: Option<String> }
impl Hitbtc3Exchange { pub fn new() -> Self { Self::default() } pub fn with_auth(k:&str,s:&str)->Self{Self{api_key:Some(k.into()),secret:Some(s.into())}} }

#[async_trait]
impl Exchange for Hitbtc3Exchange {
    async fn fetch_ticker(&self, symbol:&str) -> CcxtResult<Ticker> {
        let url = format!("{}", base());
        let resp = reqwest::get(&url).await.map_err(CcxtError::Network)?;
        let data: Value = resp.json().await.map_err(|e| CcxtError::Parse(e.to_string()))?;
        Ok(Ticker{symbol:symbol.into(),timestamp:Utc::now().timestamp(),high:0.0,low:0.0,bid:0.0,ask:0.0,last:0.0,volume:0.0,open:None,close:None})
    }
    async fn fetch_order_book(&self, symbol:&str, limit:Option<u32>) -> CcxtResult<OrderBook> {
        let url = format!("{}", base());
        let resp = reqwest::get(&url).await.map_err(CcxtError::Network)?;
        let _data: Value = resp.json().await.map_err(|e| CcxtError::Parse(e.to_string()))?;
        Ok(OrderBook{symbol:symbol.into(),timestamp:Utc::now().timestamp(),bids:vec![],asks:vec![]})
    }
    async fn fetch_balance(&self) -> CcxtResult<Balance> { Err(CcxtError::AuthRequired) }
    async fn create_order(&self,_s:&str,_side:OrderSide,_amount:f64,_price:Option<f64>) -> CcxtResult<Order> { Err(CcxtError::AuthRequired) }
    fn id(&self) -> &'static str { "hitbtc3" }
    fn is_implemented(&self) -> bool { true }
}
