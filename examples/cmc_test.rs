use rust_ccxt::CoinMarketCap;
#[tokio::main]
async fn main() {
    let cmc = CoinMarketCap::new("fb0db966-e378-44c0-8408-f3ed1cbea079");
    match cmc.quotes("BTC,ETH").await {
        Ok(v) => {
            if let Some(data) = v["data"].as_object() {
                for (sym, info) in data {
                    println!("{}: ${}", sym, info["quote"]["USD"]["price"]);
                }
            }
        }
        Err(e) => println!("err: {}", e),
    }
}
