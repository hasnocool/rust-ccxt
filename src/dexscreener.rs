//! DexScreener API (free, no auth).
//! https://docs.dexscreener.com/api/reference

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct DexPair {
    pub chain_id: String,
    pub dex_id: String,
    pub pair_address: String,
    pub base_token: String,
    pub quote_token: String,
    pub price_usd: f64,
    pub volume_24h: f64,
    pub liquidity_usd: f64,
    pub price_change_24h: f64,
}

#[derive(Deserialize)]
struct DexResponse {
    pairs: Option<Vec<DexPairRaw>>,
}

#[derive(Deserialize)]
struct DexPairRaw {
    #[serde(rename = "chainId")]
    chain_id: Option<String>,
    #[serde(rename = "dexId")]
    dex_id: Option<String>,
    #[serde(rename = "pairAddress")]
    pair_address: Option<String>,
    #[serde(rename = "baseToken")]
    base_token: Option<DexToken>,
    #[serde(rename = "quoteToken")]
    quote_token: Option<DexToken>,
    #[serde(rename = "priceUsd")]
    price_usd: Option<String>,
    #[serde(rename = "volume")]
    volume: Option<DexVolume>,
    liquidity: Option<DexLiquidity>,
    #[serde(rename = "priceChange")]
    price_change: Option<DexPriceChange>,
}

#[derive(Deserialize)]
struct DexToken {
    symbol: Option<String>,
    address: Option<String>,
}

#[derive(Deserialize)]
struct DexVolume {
    h24: Option<f64>,
}

#[derive(Deserialize)]
struct DexLiquidity {
    usd: Option<f64>,
}

#[derive(Deserialize)]
struct DexPriceChange {
    h24: Option<f64>,
}

pub struct DexScreener;

impl DexScreener {
    /// Search pairs by token symbol or address.
    pub async fn search(query: &str) -> Result<Vec<DexPair>, reqwest::Error> {
        let url = format!("https://api.dexscreener.com/latest/dex/search?q={}", urlencoding_simple(query));
        let resp = reqwest::get(&url).await?;
        let raw: DexResponse = resp.json().await?;
        Ok(raw.pairs.unwrap_or_default().into_iter().map(|p| DexPair {
            chain_id: p.chain_id.unwrap_or_default(),
            dex_id: p.dex_id.unwrap_or_default(),
            pair_address: p.pair_address.unwrap_or_default(),
            base_token: p.base_token.and_then(|t| t.symbol).unwrap_or_default(),
            quote_token: p.quote_token.and_then(|t| t.symbol).unwrap_or_default(),
            price_usd: p.price_usd.and_then(|s| s.parse().ok()).unwrap_or(0.0),
            volume_24h: p.volume.and_then(|v| v.h24).unwrap_or(0.0),
            liquidity_usd: p.liquidity.and_then(|l| l.usd).unwrap_or(0.0),
            price_change_24h: p.price_change.and_then(|pc| pc.h24).unwrap_or(0.0),
        }).collect())
    }

    /// Get pair by chain + address.
    pub async fn pair(chain: &str, address: &str) -> Result<Option<DexPair>, reqwest::Error> {
        let url = format!("https://api.dexscreener.com/latest/dex/pairs/{}/{}", chain, address);
        let resp = reqwest::get(&url).await?;
        let raw: Value = resp.json().await?;
        let pairs = raw["pairs"].as_array();
        if let Some(arr) = pairs {
            if let Some(p) = arr.first() {
                return Ok(Some(DexPair {
                    chain_id: p["chainId"].as_str().unwrap_or("").to_string(),
                    dex_id: p["dexId"].as_str().unwrap_or("").to_string(),
                    pair_address: p["pairAddress"].as_str().unwrap_or("").to_string(),
                    base_token: p["baseToken"]["symbol"].as_str().unwrap_or("").to_string(),
                    quote_token: p["quoteToken"]["symbol"].as_str().unwrap_or("").to_string(),
                    price_usd: p["priceUsd"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    volume_24h: p["volume"]["h24"].as_f64().unwrap_or(0.0),
                    liquidity_usd: p["liquidity"]["usd"].as_f64().unwrap_or(0.0),
                    price_change_24h: p["priceChange"]["h24"].as_f64().unwrap_or(0.0),
                }));
            }
        }
        Ok(None)
    }

    /// Get token profiles (trending).
    pub async fn token_profiles() -> Result<Value, reqwest::Error> {
        let url = "https://api.dexscreener.com/token-profiles/latest/v1";
        let resp = reqwest::get(url).await?;
        Ok(resp.json().await?)
    }
}

fn urlencoding_simple(s: &str) -> String {
    s.chars().map(|c| match c {
        ' ' => "+".to_string(),
        c if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' => c.to_string(),
        c => format!("%{:02X}", c as u8),
    }).collect()
}
