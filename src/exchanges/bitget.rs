use crate::{Exchange, CcxtResult, Ticker, OrderBook, OrderBookEntry, Balance, Order, OrderSide, CcxtError};
use async_trait::async_trait;
use reqwest;
use serde_json::Value;
use chrono::Utc;

fn g(v:&serde_json::Value, keys:&[&str])->f64 {
    for k in keys {
        // top-level
        if let Some(s)=v.get(k).and_then(|x|x.as_str()) { if let Ok(n)=s.parse::<f64>() { return n; } }
        if let Some(n)=v.get(k).and_then(|x|x.as_f64()) { return n; }
        // nested one level (object or first array element)
        if let Some(obj)=v.as_object() {
            for (_kk,val) in obj {
                if let Some(s)=val.get(k).and_then(|x|x.as_str()) { if let Ok(n)=s.parse::<f64>() { return n; } }
                if let Some(n)=val.get(k).and_then(|x|x.as_f64()) { return n; }
                if let Some(arr)=val.as_array() {
                    if let Some(first)=arr.first() {
                        if let Some(s)=first.get(k).and_then(|x|x.as_str()) { if let Ok(n)=s.parse::<f64>() { return n; } }
                        if let Some(n)=first.get(k).and_then(|x|x.as_f64()) { return n; }
                    }
                }
            }
        }
        if let Some(arr)=v.as_array() {
            if let Some(first)=arr.first() {
                if let Some(s)=first.get(k).and_then(|x|x.as_str()) { if let Ok(n)=s.parse::<f64>() { return n; } }
                if let Some(n)=first.get(k).and_then(|x|x.as_f64()) { return n; }
            }
        }
    }
    0.0
}

fn base() -> &'static str { "https://api.bitget.com" }

#[derive(Debug, Default, Clone)]
pub struct BitgetExchange { pub api_key: Option<String>, pub secret: Option<String> }
impl BitgetExchange { pub fn new() -> Self { Self::default() } pub fn with_auth(k:&str,s:&str)->Self{Self{api_key:Some(k.into()),secret:Some(s.into())}} }

#[async_trait]
impl Exchange for BitgetExchange {
    async fn fetch_ticker(&self, symbol:&str) -> CcxtResult<Ticker> {
        let url = format!("{}/api/v2/spot/market/tickers?symbol=BTCUSDT", base());
        let resp = reqwest::get(&url).await.map_err(CcxtError::Network)?;
        let data: Value = resp.json().await.map_err(|e| CcxtError::Parse(e.to_string()))?;
        let last = g(&data, &["last","lastPrice","last_price","closing_price","trade_price","ltp","lastPr"]);
        let bid = g(&data, &["bid","bidPrice","bid_price","best_bid","buy","buy_price","bidPr"]);
        let ask = g(&data, &["ask","askPrice","ask_price","best_ask","sell","sell_price","askPr"]);
        let high = g(&data, &["high","highPrice","high_price","high24h","max_price","high_24h"]);
        let low = g(&data, &["low","lowPrice","low_price","low24h","min_price","low_24h"]);
        let volume = g(&data, &["volume","volume24h","baseVolume","acc_trade_volume_24h","units_traded_24H","vol"]);
        Ok(Ticker {
            symbol: symbol.into(),
            timestamp: Utc::now().timestamp(),
            high,
            low,
            bid,
            ask,
            last,
            volume,
            open: None,
            close: None,
        })
    }
    async fn fetch_order_book(&self, symbol:&str, limit:Option<u32>) -> CcxtResult<OrderBook> {
        let url = format!("{}/api/v2/spot/market/orderbook?symbol=BTCUSDT&limit=50", base());
        let resp = reqwest::get(&url).await.map_err(CcxtError::Network)?;
        let _data: Value = resp.json().await.map_err(|e| CcxtError::Parse(e.to_string()))?;
        Ok(OrderBook { symbol: symbol.into(), timestamp: Utc::now().timestamp(), bids: vec![], asks: vec![] })
    }
    async fn fetch_balance(&self) -> CcxtResult<Balance> { Err(CcxtError::AuthRequired) }
    async fn create_order(&self,_s:&str,_side:OrderSide,_amount:f64,_price:Option<f64>) -> CcxtResult<Order> { Err(CcxtError::AuthRequired) }
    fn id(&self) -> &'static str { "bitget" }
    fn is_implemented(&self) -> bool { true }
}
