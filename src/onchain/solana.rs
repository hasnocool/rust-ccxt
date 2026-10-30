//! Solana on-chain data via free public RPC.
use serde_json::Value;
pub async fn get_solana_total_supply() -> Result<Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let body = serde_json::json!({ "jsonrpc": "2.0", "id": 1, "method": "getSupply", "params": [] });
    let resp = client.post("https://api.mainnet-beta.solana.com").json(&body).send().await?;
    Ok(resp.json().await?)
}
