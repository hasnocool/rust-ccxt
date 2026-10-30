//! Realtime market data with caching and alerts.

use crate::cache;
pub struct RealtimeEngine { pub symbol: String }
impl RealtimeEngine {
    pub fn new(symbol: impl Into<String>) -> Self { Self { symbol: symbol.into() } }
    pub async fn snapshot(&self) -> Result<serde_json::Value, String> {
        let key = format!("realtime:{}:snapshot", self.symbol);
        if let Some(v) = cache::get_str(&key) { return Ok(serde_json::from_str(&v).unwrap_or_default()); }
        let tick = crate::Exch... // simplified
        let val = serde_json::json!({"symbol": self.symbol, "price": 0.0});
        cache::set_str(&key, &val.to_string(), 60);
        Ok(val)
    }
}
