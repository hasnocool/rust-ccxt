use rust_ccxt::stocks;
#[tokio::main]
async fn main() {
    let av = stocks::StockClient::alpha_vantage("0A32OAYKCCTH3RPF");
    match av.quote("AAPL").await {
        Ok(q) => println!("AV AAPL: ${:.2}", q.price),
        Err(e) => println!("AV err: {}", e),
    }
    let fred = stocks::StockClient::fred("d292b936f214d28eff85bab336c5b828");
    match fred.series("GDP", 2).await {
        Ok(v) => println!("FRED GDP: {}", v["observations"][0]["value"]),
        Err(e) => println!("FRED err: {}", e),
    }
    println!("stocks module OK");
}
