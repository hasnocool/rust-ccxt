//! GeckoTerminal API (free, no auth).
//! https://www.geckoterminal.com/dex-api

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct GtPool {
    pub address: String,
    pub name: String,
    pub base_token: String,
    pub quote_token: String,
    pub price_usd: f64,
    pub volume_24h: f64,
    pub reserve_usd: f64,
    pub price_change_24h: f64,
}

pub struct GeckoTerminal;

impl GeckoTerminal {
    /// Network list.
    pub async fn networks() -> Result<Value, reqwest::Error> {
        let url = "https://api.geckoterminal.com/api/v2/networks";
        let resp = reqwest::get(url).await?;
        Ok(resp.json().await?)
    }

    /// Top pools on a network.
    pub async fn top_pools(network: &str, page: u32) -> Result<Vec<GtPool>, reqwest::Error> {
        let url = format!(
            "https://api.geckoterminal.com/api/v2/networks/{}/pools?page={}",
            network, page
        );
        let resp = reqwest::get(&url).await?;
        let v: Value = resp.json().await?;
        let data = v["data"].as_array().cloned().unwrap_or_default();
        Ok(data.into_iter().map(|p| {
            let a = &p["attributes"];
            GtPool {
                address: p["id"].as_str().unwrap_or("").to_string(),
                name: a["name"].as_str().unwrap_or("").to_string(),
                base_token: a["base_token_price_usd"].as_str().unwrap_or("").to_string(),
                quote_token: a["quote_token_price_usd"].as_str().unwrap_or("").to_string(),
                price_usd: a["base_token_price_usd"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                volume_24h: a["volume_usd"]["h24"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                reserve_usd: a["reserve_in_usd"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                price_change_24h: a["price_change_percentage"]["h24"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            }
        }).collect())
    }

    /// Search pools by query.
    pub async fn search(query: &str) -> Result<Value, reqwest::Error> {
        let url = format!(
            "https://api.geckoterminal.com/api/v2/search/pools?query={}",
            urlencoding_simple(query)
        );
        let resp = reqwest::get(&url).await?;
        Ok(resp.json().await?)
    }

    /// Token info.
    pub async fn token(network: &str, address: &str) -> Result<Value, reqwest::Error> {
        let url = format!(
            "https://api.geckoterminal.com/api/v2/networks/{}/tokens/{}",
            network, address
        );
        let resp = reqwest::get(&url).await?;
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
