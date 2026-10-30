use rust_ccxt::{Exchange, exchanges::*};
#[tokio::main]
async fn main() {
    let adapters: Vec<Box<dyn Exchange + Send + Sync>> = vec![
        Box::new(bybit::BybitExchange::new()),
        Box::new(kraken::KrakenExchange::new()),
        Box::new(coinbasepro::CoinbaseproExchange::new()),
        Box::new(gateio::GateioExchange::new()),
        Box::new(okex::OkexExchange::new()),
        Box::new(binance::BinanceExchange::new()),
        Box::new(bitfinex::BitfinexExchange::new()),
        Box::new(huobi::HuobiExchange::new()),
        Box::new(kucoin::KucoinExchange::new()),
        Box::new(bitstamp::BitstampExchange::new()),
        Box::new(gemini::GeminiExchange::new()),
        Box::new(bittrex::BittrexExchange::new()),
        Box::new(poloniex::PoloniexExchange::new()),
        Box::new(bitmex::BitmexExchange::new()),
        Box::new(deribit::DeribitExchange::new()),
        Box::new(hitbtc::HitbtcExchange::new()),
        Box::new(mexc::MexcExchange::new()),
        Box::new(phemex::PhemexExchange::new()),
        Box::new(lbank::LbankExchange::new()),
        Box::new(cryptocom::CryptocomExchange::new()),
    ];
    for (i, a) in adapters.iter().enumerate() {
        let sym = match a.id() {
            "bitmex" | "deribit" => "BTCUSD",
            "kraken" => "BTC/USD",
            _ => "BTCUSDT",
        };
        let res = a.fetch_ticker(sym).await;
        match res {
            Ok(t) => println!("{}: OK last={}", a.id(), t.last),
            Err(e) => println!("{}: ERR {}", a.id(), e),
        }
    }
}
