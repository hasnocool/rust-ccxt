use rust_ccxt::exchanges::binance::BinanceExchange;
use rust_ccxt::Exchange;
#[tokio::main]
async fn main() {
    let ex = BinanceExchange::new();
    match ex.fetch_ticker("BTCUSDT").await {
        Ok(t) => println!("Binance BTCUSDT: ${:.2}", t.last),
        Err(e) => println!("err: {}", e),
    }
}
