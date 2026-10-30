use rust_ccxt::CoinMarketCap;
use rust_ccxt::onchain::Santiment;
#[tokio::main]
async fn main() {
    println!("=== CoinMarketCap ===");
    let cmc = CoinMarketCap::new("fb0db966-e378-44c0-8408-f3ed1cbea079");
    let v = cmc.quotes("BTC,ETH,SOL").await.unwrap();
    for (sym, data) in v["data"].as_object().unwrap() {
        println!("{} ${}", sym, data["quote"]["USD"]["price"]);
    }
    println!("\n=== Santiment ===");
    let s = Santiment::new("y6g7f52e72k55coy_fu4sfxhbqjqrmenb");
    let v = s.active_addresses("bitcoin", "2026-10-25T00:00:00Z", "2026-10-30T00:00:00Z").await.unwrap();
    println!("Santiment: {}", v);
}
