use rust_ccxt::{Onchain, Derivatives, correlation::CorrelationEngine};
use rust_ccxt_news::{NewsClient, Sentiment};

#[tokio::main]
async fn main() {
    // 1. On-chain snapshot
    let onchain = Onchain::snapshot().await.unwrap_or_else(|_| {
        eprintln!("onchain fetch failed");
        // fallback empty
        rust_ccxt::OnchainSnapshot {
            btc_price_usd: 0.0, hashrate_ehs: 0.0, mempool_tx_count: 0,
            mempool_vsize: 0, fee_fast_sat_vb: 0.0, fee_medium_sat_vb: 0.0,
            fee_slow_sat_vb: 0.0, block_height: 0,
        }
    });
    println!("BTC price: ${:.2}", onchain.btc_price_usd);
    println!("Mempool tx: {}, vsize: {}", onchain.mempool_tx_count, onchain.mempool_vsize);
    println!("Fees (fast/med/slow): {:.1}/{:.1}/{:.1} sat/vB", onchain.fee_fast_sat_vb, onchain.fee_medium_sat_vb, onchain.fee_slow_sat_vb);

    // 2. Derivatives
    let funding = Derivatives::binance_funding("BTCUSDT").await.unwrap_or_else(|_| {
        rust_ccxt::FundingRate { symbol: "BTCUSDT".into(), funding_rate: 0.0, funding_time: 0 }
    });
    println!("BTC funding rate: {:.6}", funding.funding_rate);

    // 3. News sentiment
    let news = NewsClient::google_news("bitcoin").await.unwrap_or_default();
    let sentiment = Sentiment::average_items(&news);
    println!("News items: {}, avg sentiment: {:.3}", news.len(), sentiment);

    // 4. Correlation
    let price_change_pct = 2.5; // placeholder
    let signal = CorrelationEngine::analyze(price_change_pct, sentiment, funding.funding_rate, &onchain);
    println!("\nComposite signal: {:.3} ({})", signal.composite_score, signal.interpretation());
}
