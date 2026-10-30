//! Per-exchange rate limiter (token bucket).

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

#[derive(Clone)]
struct Bucket { tokens: f64, last: Instant, rate: f64, capacity: f64 }

static BUCKETS: OnceLock<Mutex<HashMap<String, Bucket>>> = OnceLock::new();
fn buckets() -> &'static Mutex<HashMap<String, Bucket>> {
    BUCKETS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub struct RateLimiter;

impl RateLimiter {
    /// Configure a bucket for an exchange.
    pub fn configure(exchange: &str, requests_per_second: f64, burst: f64) {
        let mut b = buckets().lock().unwrap();
        b.insert(exchange.to_string(), Bucket { tokens: burst, last: Instant::now(), rate: requests_per_second, capacity: burst });
    }

    /// Wait until a token is available for the exchange.
    pub async fn acquire(exchange: &str) {
        loop {
            let wait = {
                let mut b = buckets().lock().unwrap();
                let bucket = b.entry(exchange.to_string()).or_insert(Bucket { tokens: 10.0, last: Instant::now(), rate: 5.0, capacity: 10.0 });
                let now = Instant::now();
                let elapsed = now.duration_since(bucket.last).as_secs_f64();
                bucket.tokens = (bucket.tokens + elapsed * bucket.rate).min(bucket.capacity);
                bucket.last = now;
                if bucket.tokens >= 1.0 {
                    bucket.tokens -= 1.0;
                    return;
                }
                let need = 1.0 - bucket.tokens;
                Duration::from_secs_f64(need / bucket.rate)
            };
            tokio::time::sleep(wait).await;
        }
    }
}
