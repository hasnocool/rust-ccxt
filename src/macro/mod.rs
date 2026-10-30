//! Macro economic data from FRED, using API key from APIKEYS.md.
pub struct MacroClient { pub api_key: String }
impl MacroClient {
    pub fn new(api_key: impl Into<String>) -> Self { Self { api_key: api_key.into() } }
    pub async fn fred_latest(&self, series: &str) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("https://api.stlouisfed.org/fred/series/observations?series_id={}&api_key={}&file_type=json&limit=1", series, self.api_key);
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
        let resp = reqwest::get(&url).await?.json().await?;
        Ok(resp)
    }
}
