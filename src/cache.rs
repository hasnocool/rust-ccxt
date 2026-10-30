//! Simple in-memory TTL cache for API responses.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

#[derive(Clone)]
struct Entry {
    value: String,
    expires: Instant,
}

static CACHE: OnceLock<Mutex<HashMap<String, Entry>>> = OnceLock::new();

fn cache() -> &'static Mutex<HashMap<String, Entry>> {
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Get cached value if not expired.
pub fn get(key: &str) -> Option<String> {
    let mut c = cache().lock().unwrap();
    if let Some(e) = c.get(key) {
        if Instant::now() < e.expires {
            return Some(e.value.clone());
        }
        c.remove(key);
    }
    None
}

/// Set cached value with TTL.
pub fn set(key: &str, value: String, ttl: Duration) {
    let mut c = cache().lock().unwrap();
    c.insert(key.to_string(), Entry {
        value,
        expires: Instant::now() + ttl,
    });
}

/// Clear all cached entries.
pub fn clear() {
    cache().lock().unwrap().clear();
}

/// Helper: fetch with cache. Returns cached string if available, else calls fetcher and caches result.
pub async fn cached<F, Fut>(key: &str, ttl: Duration, fetcher: F) -> Result<String, reqwest::Error>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<String, reqwest::Error>>,
{
    if let Some(v) = get(key) {
        return Ok(v);
    }
    let v = fetcher().await?;
    set(key, v.clone(), ttl);
    Ok(v)
}
