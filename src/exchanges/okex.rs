//! Okex adapter.
use crate::{Exchange, CcxtResult, Ticker, OrderBook, OrderBookEntry, Balance, Order, OrderSide, CcxtError};
use async_trait::async_trait;
use reqwest;
use serde_json::Value;
use chrono::Utc;

fn base()->&'static str{ "https://www.okx.com" }
fn inst_id(symbol:&str)->String{ symbol.replace("/","-").to_uppercase() }

#[derive(Debug, Default, Clone)]
pub struct OkexExchange{ pub api_key: Option<String>, pub secret: Option<String> }
impl OkexExchange{ pub fn new()->Self{Self::default()} pub fn with_auth(k:&str,s:&str)->Self{Self{api_key:Some(k.into()),secret:Some(s.into())}} }

#[async_trait]
impl Exchange for OkexExchange{
    async fn fetch_ticker(&self, symbol:&str)->CcxtResult<Ticker>{
        let inst=inst_id(symbol);
        let url=format!("{}/api/v5/market/tickers?instId={}",base(),inst);
        let resp=reqwest::get(&url).await.map_err(CcxtError::Network)?;
        let data:Value=resp.json().await.map_err(|e|CcxtError::Parse(e.to_string()))?;
        let lst=data["data"].as_array().ok_or(CcxtError::Parse("no data".into()))?;
        let t=lst.get(0).ok_or(CcxtError::Parse("no ticker".into()))?;
        Ok(Ticker{symbol:symbol.into(),timestamp:Utc::now().timestamp(),high:t["high24h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),low:t["low24h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),bid:t["bidPx"].as_str().unwrap_or("0").parse().unwrap_or(0.0),ask:t["askPx"].as_str().unwrap_or("0").parse().unwrap_or(0.0),last:t["last"].as_str().unwrap_or("0").parse().unwrap_or(0.0),volume:t["vol24h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),open:None,close:None})
    }
    async fn fetch_order_book(&self, symbol:&str, limit:Option<u32>)->CcxtResult<OrderBook>{
        let inst=inst_id(symbol);
        let sz=limit.unwrap_or(100);
        let url=format!("{}/api/v5/market/books?instId={}&sz={}",base(),inst,sz);
        let resp=reqwest::get(&url).await.map_err(CcxtError::Network)?;
        let data:Value=resp.json().await.map_err(|e|CcxtError::Parse(e.to_string()))?;
        let arr=data["data"].as_array().ok_or(CcxtError::Parse("no data".into()))?;
        let book=arr.get(0).ok_or(CcxtError::Parse("no book".into()))?;
        let bids=book["bids"].as_array().unwrap_or(&vec![]).iter().filter_map(|e|{let a=e.as_array()?; Some(OrderBookEntry{price:a[0].as_str()?.parse().ok()?,amount:a[1].as_str()?.parse().ok()?})}).collect();
        let asks=book["asks"].as_array().unwrap_or(&vec![]).iter().filter_map(|e|{let a=e.as_array()?; Some(OrderBookEntry{price:a[0].as_str()?.parse().ok()?,amount:a[1].as_str()?.parse().ok()?})}).collect();
        Ok(OrderBook{symbol:symbol.into(),timestamp:Utc::now().timestamp(),bids,asks})
    }
    async fn fetch_balance(&self)->CcxtResult<Balance>{ Err(CcxtError::Unsupported("auth not implemented".into())) }
    async fn create_order(&self,_s:&str,_side:OrderSide,_amount:f64,_price:Option<f64>)->CcxtResult<Order>{ Err(CcxtError::Unsupported("auth not implemented".into())) }
    fn id(&self)->&'static str{"okex"}
    fn is_implemented(&self)->bool{true}
}
