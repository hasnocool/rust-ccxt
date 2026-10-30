use rust_ccxt::{Exchange, exchanges::binance::BinanceExchange};

#[tokio::main]
async fn main() {
    let ex = BinanceExchange::new();
    println!("Exchange id: {}", ex.id());

    match ex.fetch_ticker("BTC/USDT").await {
        Ok(t) => {
            println!("Ticker for {}: last={} bid={} ask={}", t.symbol, t.last, t.bid, t.ask);
        }
        Err(e) => eprintln!("Error fetching ticker: {}", e),
    }

    match ex.fetch_order_book("BTC/USDT", Some(5)).await {
        Ok(ob) => {
            println!("OrderBook bids: {}, asks: {}", ob.bids.len(), ob.asks.len());
            if let Some(b) = ob.bids.first() {
                println!("Best bid: {} {}", b.price, b.amount);
            }
            if let Some(a) = ob.asks.first() {
                println!("Best ask: {} {}", a.price, a.amount);
            }
        }
        Err(e) => eprintln!("Error fetching order book: {}", e),
    }
}
