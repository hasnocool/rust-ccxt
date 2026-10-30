//! On-chain metrics via free public endpoints (no auth).
//!
//! Sources: mempool.space, blockchain.info, CoinGecko (free tier).

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OnchainSnapshot {
    pub btc_price_usd: f64,
    pub hashrate_ehs: f64,
    pub mempool_tx_count: u64,
    pub mempool_vsize: u64,
    pub fee_fast_sat_vb: f64,
    pub fee_medium_sat_vb: f64,
    pub fee_slow_sat_vb: f64,
    pub block_height: u64,
}

#[derive(Deserialize)]
struct MempoolFees {
    #[serde(rename="fastestFee")]
    fastest_fee: f64,
    #[serde(rename="halfHourFee")]
    half_hour_fee: f64,
    #[serde(rename="hourFee")]
    hour_fee: f64,
}

pub struct Onchain;

impl Onchain {
    /// Fetch recommended fees from mempool.space.
    pub async fn fees() -> Result<(f64, f64, f64), reqwest::Error> {
        let url = "https://mempool.space/api/v1/fees/recommended";
        let resp = reqwest::get(url).await?;
        let f: MempoolFees = resp.json().await?;
        Ok((f.fastest_fee, f.half_hour_fee, f.hour_fee))
    }

    /// Fetch mempool stats from mempool.space.
    pub async fn mempool() -> Result<(u64, u64), reqwest::Error> {
        let url = "https://mempool.space/api/mempool";
        let resp = reqwest::get(url).await?;
        let v: Value = resp.json().await?;
        let count = v["count"].as_u64().unwrap_or(0);
        let vsize = v["vsize"].as_u64().unwrap_or(0);
        Ok((count, vsize))
    }

    /// Fetch current block height from mempool.space.
    pub async fn block_height() -> Result<u64, reqwest::Error> {
        let url = "https://mempool.space/api/blocks/tip/height";
        let resp = reqwest::get(url).await?;
        let text = resp.text().await?;
        Ok(text.trim().parse().unwrap_or(0))
    }

    /// Fetch BTC price from Binance public ticker (free, no key).
    pub async fn btc_price() -> Result<f64, reqwest::Error> {
        let url = "https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT";
        let resp = reqwest::get(url).await?;
        let v: Value = resp.json().await?;
        Ok(v["price"].as_str().unwrap_or("0").parse().unwrap_or(0.0))
    }

    /// Fetch hashrate from blockchain.info (TH/s).
    pub async fn hashrate() -> Result<f64, reqwest::Error> {
        let url = "https://blockchain.info/q/hashrate";
        let resp = reqwest::get(url).await?;
        let text = resp.text().await?;
        Ok(text.trim().parse().unwrap_or(0.0))
    }

    /// Ethereum gas price (gwei) via public RPC (free, no key).
    pub async fn eth_gas_price() -> Result<f64, reqwest::Error> {
        let client = reqwest::Client::new();
        let body = serde_json::json!({"jsonrpc":"2.0","method":"eth_gasPrice","params":[],"id":1});
        let resp = client.post("https://cloudflare-eth.com").json(&body).send().await?;
        let v: serde_json::Value = resp.json().await?;
        let hex = v["result"].as_str().unwrap_or("0x0");
        let wei = u64::from_str_radix(hex.trim_start_matches("0x"), 16).unwrap_or(0);
        Ok(wei as f64 / 1e9)
    }

    /// Aggregate snapshot of on-chain state (cached 60s).
    pub async fn snapshot() -> Result<OnchainSnapshot, reqwest::Error> {
        if let Some(cached) = crate::cache::get("onchain:snapshot") {
            // Re-parse cached JSON
            if let Ok(snap) = serde_json::from_str::<OnchainSnapshot>(&cached) {
                return Ok(snap);
            }
        }
        let (fast, med, slow) = Self::fees().await.unwrap_or((0.0, 0.0, 0.0));
        let (count, vsize) = Self::mempool().await.unwrap_or((0, 0));
        let height = Self::block_height().await.unwrap_or(0);
        let price = Self::btc_price().await.unwrap_or(0.0);
        let hr = Self::hashrate().await.unwrap_or(0.0);
        let snap = OnchainSnapshot {
            btc_price_usd: price,
            hashrate_ehs: hr / 1e6, // TH/s -> EH/s
            mempool_tx_count: count,
            mempool_vsize: vsize,
            fee_fast_sat_vb: fast,
            fee_medium_sat_vb: med,
            fee_slow_sat_vb: slow,
            block_height: height,
        };
        if let Ok(s) = serde_json::to_string(&snap) {
            crate::cache::set("onchain:snapshot", s, std::time::Duration::from_secs(60));
        }
        Ok(snap)
    }
}
