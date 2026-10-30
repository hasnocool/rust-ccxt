use rust_ccxt::onchain::Santiment;
#[tokio::main]
async fn main() {
    let s = Santiment::new("y6g7f52e72k55coy_fu4sfxhbqjqrmenb");
    match s.active_addresses("bitcoin", "2026-10-20T00:00:00Z", "2026-10-30T00:00:00Z").await {
        Ok(v) => println!("Santiment active_addresses: {}", v),
        Err(e) => println!("err: {}", e),
    }
}
