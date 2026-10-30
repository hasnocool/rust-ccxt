use rust_ccxt::exchanges::binance::BinanceExchange;
use rust_ccxt::{Exchange, OrderSide};

#[tokio::main]
async fn main() {
    // Demonstrate trait usage with Binance stub adapter.
    let ex = BinanceExchange::new();

    println!("Exchange id: {}", ex.id());
    println!("Implemented? {}", ex.is_implemented());

    // Example call — will panic due to stub; shows compiler wiring.
    // let ticker = ex.fetch_ticker("BTC/USDT").await.unwrap();
    // println!("{ticker:?}");

    // Verify trait bound
    fn assert_exchange<E: Exchange + Send + Sync>() {}
    assert_exchange::<BinanceExchange>();

    println!("rust-ccxt scaffold compiles with trait design.");
}
