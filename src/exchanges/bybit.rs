use crate::{Exchange, CcxtResult, Ticker, OrderBook, OrderBookEntry, Balance, Order, OrderSide, CcxtError};
use async_trait::async_trait;
use reqwest;
use serde_json::Value;
use chrono::Utc;

fn base() -> &'static str { "https://api.bybit.com" }

#[derive(Debug, Default, Clone)]
pub struct BybitExchange { pub api_key: Option<String>, pub secret: Option<String> }

impl BybitExchange {
    pub fn new() -> Self { Self::default() }
    pub fn with_auth(k:&str,s:&str)->Self{Self{api_key:Some(k.into()),secret:Some(s.into())}}

    fn parse_depth(v:&Value)->Vec<OrderBookEntry>{
        v.as_array().unwrap_or(&vec![]).iter().filter_map(|e|{
            let a=e.as_array()?; if a.len()<2 { return None; }
            let p=a[0].as_str()?.parse::<f64>().ok()?;
            let q=a[1].as_str()?.parse::<f64>().ok()?;
            Some(OrderBookEntry{price:p,amount:q})
        }).collect()
    }
}

#[async_trait]
impl Exchange for BybitExchange {
    async fn fetch_ticker(&self, symbol:&str)->CcxtResult<Ticker>{
        let pair = symbol.replace("/","");
        let url = format!("{}/v5/market/tickers?category=spot&symbol={}", base(), pair);
        let resp = reqwest::get(&url).await.map_err(CcxtError::Network)?;
        let data: Value = resp.json().await.map_err(|e| CcxtError::Parse(e.to_string()))?;
        let lst = data["result"]["list"].as_array().cloned().unwrap_or_default();
        let t = lst.into_iter().next().ok_or(CcxtError::Parse("no ticker".into()))?;
        Ok(Ticker{
            symbol:symbol.into(),
            timestamp:Utc::now().timestamp(),
            high:t["highPrice"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            low:t["lowPrice"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            bid:t["bid1Price"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            ask:t["ask1Price"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            last:t["lastPrice"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            volume:t["volume24h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            open:Some(t["openPrice"].as_str().unwrap_or("0").parse().unwrap_or(0.0)),
            close:None,
        })
    }
    async fn fetch_order_book(&self, symbol:&str, limit:Option<u32>)->CcxtResult<OrderBook>{
        let pair = symbol.replace("/","");
        let lim = limit.unwrap_or(50);
        let url = format!("{}/v5/market/orderbook?category=spot&symbol={}&limit={}", base(), pair, lim);
        let resp = reqwest::get(&url).await.map_err(CcxtError::Network)?;
        let data: Value = resp.json().await.map_err(|e| CcxtError::Parse(e.to_string()))?;
        let bids = Self::parse_depth(&data["result"]["b"]);
        let asks = Self::parse_depth(&data["result"]["a"]);
        Ok(OrderBook{symbol:symbol.into(), timestamp:Utc::now().timestamp(), bids, asks})
    }
    async fn fetch_balance(&self)->CcxtResult<Balance>{
        let api_key=self.api_key.as_ref().ok_or(CcxtError::AuthRequired)?;
        let secret=self.secret.as_ref().ok_or(CcxtError::AuthRequired)?;
        let ts=Utc::now().timestamp_millis();
        let recv_window=5000;
        let params=format!("accountType=UNIFIED&timestamp={}",ts);
        let sign=crate::bybit_sig::sign(secret, &params);
        let url=format!("{}/v5/account/wallet-balance?{}", base(), params);
        let client=reqwest::Client::new();
        let resp=client.get(&url).header("X-BAPI-API-KEY", api_key).header("X-BAPI-TIMESTAMP", ts.to_string()).header("X-BAPI-RECV-WINDOW", recv_window.to_string()).header("X-BAPI-SIGN",sign).send().await.map_err(CcxtError::Network)?;
        let data: Value = resp.json().await.map_err(|e| CcxtError::Parse(e.to_string()))?;
        if data["retCode"].as_i64().unwrap_or(-1)!=0 { return Err(CcxtError::Api{ code:data["retCode"].as_i64().unwrap_or(0) as i32, message:data["retMsg"].as_str().unwrap_or("").to_string()}); }
        // Return first coin balance placeholder
        let coins=data["result"]["list"].as_array().cloned().unwrap_or_default();
        let coins_arr=coins.get(0).and_then(|l| l["coin"].as_array()).cloned().unwrap_or_default();
        if let Some(c)=coins_arr.first(){
            let asset=c["coin"].as_str().unwrap_or("").to_string();
            let eq=c["walletBalance"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
            Ok(Balance{currency:asset, free:eq, used:0.0, total:eq})
        }else{Ok(Balance{currency:"USDT".into(), free:0.0, used:0.0, total:0.0})}
    }
    async fn create_order(&self, symbol:&str, side:OrderSide, amount:f64, price:Option<f64>)->CcxtResult<Order>{
        let api_key=self.api_key.as_ref().ok_or(CcxtError::AuthRequired)?;
        let secret=self.secret.as_ref().ok_or(CcxtError::AuthRequired)?;
        let ts=Utc::now().timestamp_millis();
        let pair=symbol.replace("/","");
        let side_str=match side{OrderSide::Buy=>"Buy",OrderSide::Sell=>"Sell"};
        let price_str=price.unwrap_or(0.0).to_string();
        let params=format!("category=spot&symbol={}&side={}&orderType=Limit&qty={}&price={}&timestamp={}",pair,side_str,amount,price_str,ts);
        let sign=crate::bybit_sig::sign(secret,&params);
        let url=format!("{}/v5/order/create",base());
        let client=reqwest::Client::new();
        let resp=client.post(url).header("X-BAPI-API-KEY",api_key).header("X-BAPI-TIMESTAMP",ts.to_string()).header("X-BAPI-RECV-WINDOW","5000").header("X-BAPI-SIGN",sign).header("Content-Type","application/json").json(&serde_json::json!({"category":"spot","symbol":pair,"side":side_str,"orderType":"Limit","qty":amount,"price":price_str})).send().await.map_err(CcxtError::Network)?;
        let data:Value=resp.json().await.map_err(|e|CcxtError::Parse(e.to_string()))?;
        if data["retCode"].as_i64().unwrap_or(-1)!=0 { return Err(CcxtError::Api{ code:data["retCode"].as_i64().unwrap_or(0) as i32, message:data["retMsg"].as_str().unwrap_or("").to_string()}); }
        Ok(Order{id:data["result"]["orderId"].as_str().unwrap_or("").to_string(), symbol:symbol.into(), side, amount, price:price.unwrap_or(0.0), status:data["result"]["orderStatus"].as_str().unwrap_or("").to_string(), timestamp:Utc::now().timestamp()})
    }
    fn id(&self)->&'static str{"bybit"}
    fn is_implemented(&self)->bool{true}
}
