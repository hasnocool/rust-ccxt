//! Derivatives data via free public endpoints (no auth).
//!
//! Sources: Binance Futures, Bybit (funding rates, open interest).

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FundingRate {
    pub symbol: String,
    pub funding_rate: f64,
    pub funding_time: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OpenInterest {
    pub symbol: String,
    pub open_interest: f64,
    pub timestamp: i64,
}

#[derive(Deserialize)]
struct BinanceFunding {
    #[serde(rename = "fundingRate")]
    funding_rate: String,
    #[serde(rename = "fundingTime")]
    funding_time: i64,
    symbol: String,
}

#[derive(Deserialize)]
struct BinanceOI {
    #[serde(rename = "openInterest")]
    open_interest: String,
    symbol: String,
    time: i64,
}

pub struct Derivatives;

impl Derivatives {
    /// Binance Futures funding rate (latest).
    pub async fn binance_funding(symbol: &str) -> Result<FundingRate, reqwest::Error> {
        let key = format!("deriv:binance_funding:{}", symbol);
        if let Some(cached) = crate::cache::get(&key) {
            if let Ok(fr) = serde_json::from_str::<FundingRate>(&cached) {
                return Ok(fr);
            }
        }
        let url = format!(
            "https://fapi.binance.com/fapi/v1/fundingRate?symbol={}&limit=1",
            symbol
        );
        let resp = reqwest::get(&url).await?;
        let arr: Vec<BinanceFunding> = resp.json().await?;
        if let Some(f) = arr.into_iter().next() {
            let fr = FundingRate {
                symbol: f.symbol,
                funding_rate: f.funding_rate.parse().unwrap_or(0.0),
                funding_time: f.funding_time,
            };
            if let Ok(s) = serde_json::to_string(&fr) { crate::cache::set(&key, s, std::time::Duration::from_secs(60)); }
            Ok(fr)
        } else {
            Ok(FundingRate {
                symbol: symbol.to_string(),
                funding_rate: 0.0,
                funding_time: 0,
            })
        }
    }

    /// Binance Futures open interest.
    pub async fn binance_open_interest(symbol: &str) -> Result<OpenInterest, reqwest::Error> {
        let url = format!(
            "https://fapi.binance.com/fapi/v1/openInterest?symbol={}",
            symbol
        );
        let resp = reqwest::get(&url).await?;
        let oi: BinanceOI = resp.json().await?;
        Ok(OpenInterest {
            symbol: oi.symbol,
            open_interest: oi.open_interest.parse().unwrap_or(0.0),
            timestamp: oi.time,
        })
    }

    /// Bybit funding rate history (latest).
    pub async fn bybit_funding(symbol: &str) -> Result<FundingRate, reqwest::Error> {
        let url = format!(
            "https://api.bybit.com/v5/market/funding/history?category=linear&symbol={}&limit=1",
            symbol
        );
        let resp = reqwest::get(&url).await?;
        let v: Value = resp.json().await?;
        let item = v["result"]["list"].as_array().and_then(|a| a.first()).cloned();
        if let Some(it) = item {
            Ok(FundingRate {
                symbol: symbol.to_string(),
                funding_rate: it["fundingRate"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                funding_time: it["fundingRateTimestamp"].as_str().unwrap_or("0").parse().unwrap_or(0),
            })
        } else {
            Ok(FundingRate {
                symbol: symbol.to_string(),
                funding_rate: 0.0,
                funding_time: 0,
            })
        }
    }

    /// Bybit open interest.
    pub async fn bybit_open_interest(symbol: &str) -> Result<OpenInterest, reqwest::Error> {
        let url = format!(
            "https://api.bybit.com/v5/market/open-interest?category=linear&symbol={}&intervalTime=5min&limit=1",
            symbol
        );
        let resp = reqwest::get(&url).await?;
        let v: Value = resp.json().await?;
        let item = v["result"]["list"].as_array().and_then(|a| a.first()).cloned();
        if let Some(it) = item {
            Ok(OpenInterest {
                symbol: symbol.to_string(),
                open_interest: it["openInterest"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                timestamp: it["timestamp"].as_str().unwrap_or("0").parse().unwrap_or(0),
            })
        } else {
            Ok(OpenInterest {
                symbol: symbol.to_string(),
                open_interest: 0.0,
                timestamp: 0,
            })
        }
    }
}
