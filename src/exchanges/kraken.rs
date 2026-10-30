use crate::{Exchange, CcxtResult, Ticker, OrderBook, OrderBookEntry, Balance, CcxtError};
use async_trait::async_trait;
use reqwest;
use serde_json::Value;
use chrono::Utc;

fn base()->&'static str{ "https://api.kraken.com" }

fn kraken_pair(symbol:&str)->String{
    let parts = symbol.split('/').collect::<Vec<_>>();
    if parts.len()!=2 { return symbol.to_uppercase(); }
    let base=parts[0].to_uppercase();
    let quote=parts[1].to_uppercase();
    let base_map=match base.as_str(){ "BTC"=>"XBT","ETH"=>"XETH","LTC"=>"XLTC", _=>base.as_str() };
    let quote_map=match quote.as_str(){ "USD"=>"ZUSD","EUR"=>"ZEUR", _=>quote.as_str() };
    format!("{}{}",base_map,quote_map)
}

#[derive(Debug, Default, Clone)]
pub struct KrakenExchange{ pub api_key: Option<String>, pub secret: Option<String> }
impl KrakenExchange{ pub fn new()->Self{Self::default()} pub fn with_auth(k:&str,s:&str)->Self{Self{api_key:Some(k.into()),secret:Some(s.into())}} }

#[async_trait]
impl Exchange for KrakenExchange {
    async fn fetch_ticker(&self, symbol:&str)->CcxtResult<Ticker>{
        let pair=kraken_pair(symbol);
        let url=format!("{}/0/public/Ticker?pair={}",base(),pair);
        let resp=reqwest::get(&url).await.map_err(CcxtError::Network)?;
        let data:Value=resp.json().await.map_err(|e|CcxtError::Parse(e.to_string()))?;
        let res=data["result"].as_object().ok_or(CcxtError::Parse("no result".into()))?;
        let k=res.keys().next().ok_or(CcxtError::Parse("no pair".into()))?;
        let t=&res[k];
        Ok(Ticker{symbol:symbol.into(),timestamp:Utc::now().timestamp(),high:t["h"][0].as_str().unwrap_or("0").parse().unwrap_or(0.0),low:t["l"][0].as_str().unwrap_or("0").parse().unwrap_or(0.0),bid:t["b"][0].as_str().unwrap_or("0").parse().unwrap_or(0.0),ask:t["a"][0].as_str().unwrap_or("0").parse().unwrap_or(0.0),last:t["c"][0].as_str().unwrap_or("0").parse().unwrap_or(0.0),volume:t["v"][1].as_str().unwrap_or("0").parse().unwrap_or(0.0),open:None,close:None})
    }
    async fn fetch_order_book(&self, symbol:&str, limit:Option<u32>)->CcxtResult<OrderBook>{
        let pair=kraken_pair(symbol);
        let lim=limit.unwrap_or(100);
        let url=format!("{}/0/public/Depth?pair={}&count={}",base(),pair,lim);
        let resp=reqwest::get(&url).await.map_err(CcxtError::Network)?;
        let data:Value=resp.json().await.map_err(|e|CcxtError::Parse(e.to_string()))?;
        let res=data["result"].as_object().ok_or(CcxtError::Parse("no result".into()))?;
        let k=res.keys().next().ok_or(CcxtError::Parse("no pair".into()))?;
        let ob=&res[k];
        let bids:Vec<OrderBookEntry>=ob["b"].as_array().unwrap_or(&vec![]).iter().filter_map(|e|e.as_array().and_then(|a|Some(OrderBookEntry{price:a[0].as_str()?.parse().ok()?,amount:a[1].as_str()?.parse().ok()?}))).collect();
        let asks:Vec<OrderBookEntry>=ob["a"].as_array().unwrap_or(&vec![]).iter().filter_map(|e|e.as_array().and_then(|a|Some(OrderBookEntry{price:a[0].as_str()?.parse().ok()?,amount:a[1].as_str()?.parse().ok()?}))).collect();
        Ok(OrderBook{symbol:symbol.into(),timestamp:Utc::now().timestamp(),bids,asks})
    }
    async fn fetch_balance(&self)->CcxtResult<Balance>{ Err(CcxtError::Unsupported("Kraken auth not implemented".into())) }
    async fn create_order(&self,_s:&str,_side:crate::OrderSide,_amount:f64,_price:Option<f64>)->CcxtResult<crate::Order>{ Err(CcxtError::Unsupported("Kraken auth not implemented".into())) }
    fn id(&self)->&'static str{"kraken"}
    fn is_implemented(&self)->bool{true}
}
