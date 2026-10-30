use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Order side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Ticker data for a symbol.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ticker {
    pub symbol: String,
    pub timestamp: i64,
    pub high: f64,
    pub low: f64,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub volume: f64,
    pub open: Option<f64>,
    pub close: Option<f64>,
}

/// Order book entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderBookEntry {
    pub price: f64,
    pub amount: f64,
}

/// Order book snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderBook {
    pub symbol: String,
    pub timestamp: i64,
    pub bids: Vec<OrderBookEntry>,
    pub asks: Vec<OrderBookEntry>,
}

/// Account balance for one currency.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Balance {
    pub currency: String,
    pub free: f64,
    pub used: f64,
    pub total: f64,
}

/// Full balance response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BalanceMap {
    pub balances: HashMap<String, Balance>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub symbol: String,
    pub side: OrderSide,
    pub amount: f64,
    pub price: f64,
    pub status: String,
    pub timestamp: i64,
}
