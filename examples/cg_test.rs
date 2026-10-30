use rust_ccxt::CoinGecko;
#[tokio::main]
async fn main() {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    match CoinGecko::markets("usd", 3, 1).await {
        Ok(coins) => { for c in &coins { println!("{}: ${:.2}", c.symbol, c.price_usd); } }
        Err(e) => println!("err: {}", e),
    }
    match CoinGecko::trending().await {
        Ok(v) => println!("trending ok: {}", v["coins"].as_array().map(|a|a.len()).unwrap_or(0)),
        Err(e) => println!("trending err: {}", e),
    }
}
