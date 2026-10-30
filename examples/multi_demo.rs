use rust_ccxt::{Exchange, exchanges::{bybit::BybitExchange, kraken::KrakenExchange, coinbasepro::CoinbaseproExchange}};

#[tokio::main]
async fn main() {
    for ex in [
        &BybitExchange::new() as &dyn Exchange,
        &KrakenExchange::new(),
        &CoinbaseproExchange::new(),
    ] {
        println!("=== {} ===", ex.id());
        match ex.fetch_ticker("BTC/USDT").await {
            Ok(t) => println!("Ticker last={} bid={} ask={}", t.last, t.bid, t.ask),
            Err(e) => println!("Ticker error: {}", e),
        }
        match ex.fetch_order_book("BTC/USDT", Some(1)).await {
            Ok(ob) => println!("OrderBook bids={} asks={}", ob.bids.len(), ob.asks.len()),
            Err(e) => println!("OB error: {}", e),
        }
    }
}
