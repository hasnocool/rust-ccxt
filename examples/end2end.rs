use rust_ccxt::{
    onchain::Onchain, derivatives::Derivatives, correlation::CorrelationEngine,
    dexscreener::DexScreener, coingecko::CoinGecko, geckoterminal::GeckoTerminal,
    news, sentiment::Sentiment, stocks,
};
use rust_ccxt::exchanges::binance::BinanceExchange;
use rust_ccxt::Exchange;

#[tokio::main]
async fn main() {
    println!("=== rustCCXT End-to-End Pipeline ===\n");

    // 1. Exchange price (Binance BTCUSDT)
    let ex = BinanceExchange::new();
    let ticker = ex.fetch_ticker("BTCUSDT").await;
    let price = ticker.map(|t| t.last).unwrap_or(0.0);
    println!("[1] Exchange (Binance) BTCUSDT: ${:.2}", price);

    // 2. On-chain
    let onchain = Onchain::snapshot().await.unwrap_or_else(|_| rust_ccxt::onchain::OnchainSnapshot {
        btc_price_usd: 0.0, hashrate_ehs: 0.0, mempool_tx_count: 0, mempool_vsize: 0,
        fee_fast_sat_vb: 0.0, fee_medium_sat_vb: 0.0, fee_slow_sat_vb: 0.0, block_height: 0,
    });
    println!("[2] On-chain: BTC ${:.2}, mempool {} tx, fees {:.1}/{:.1}/{:.1} sat/vB",
        onchain.btc_price_usd, onchain.mempool_tx_count, onchain.fee_fast_sat_vb, onchain.fee_medium_sat_vb, onchain.fee_slow_sat_vb);

    // 3. Derivatives
    let funding = Derivatives::binance_funding("BTCUSDT").await.unwrap_or_else(|_| rust_ccxt::derivatives::FundingRate {
        symbol: "BTCUSDT".into(), funding_rate: 0.0, funding_time: 0,
    });
    println!("[3] Derivatives: BTC funding rate {:.6}", funding.funding_rate);

    // 4. DEX (DexScreener)
    match DexScreener::search("BTC").await {
        Ok(pairs) => println!("[4] DEX (DexScreener): {} BTC pairs", pairs.len()),
        Err(e) => println!("[4] DEX error: {}", e),
    }

    // 5. CoinGecko
    match CoinGecko::price("bitcoin", "usd").await {
        Ok(v) => println!("[5] CoinGecko BTC: ${}", v["bitcoin"]["usd"]),
        Err(e) => println!("[5] CoinGecko error: {}", e),
    }

    // 6. GeckoTerminal
    match GeckoTerminal::top_pools("eth", 1).await {
        Ok(pools) => println!("[6] GeckoTerminal: {} ETH pools", pools.len()),
        Err(e) => println!("[6] GeckoTerminal error: {}", e),
    }

    // 7. News sentiment
    let news_items = news::NewsClient::google_news("bitcoin").await.unwrap_or_default();
    let sentiment = Sentiment::average_items(&news_items);
    println!("[7] News: {} items, sentiment {:.3}", news_items.len(), sentiment);

    // 8. Stocks
    let av = stocks::StockClient::alpha_vantage("0A32OAYKCCTH3RPF");
    match av.quote("AAPL").await {
        Ok(q) => println!("[8] Stocks (AAPL): ${:.2}", q.price),
        Err(e) => println!("[8] Stocks error: {}", e),
    }

    // 9. Correlation
    let price_change = 2.5;
    let signal = CorrelationEngine::analyze(price_change, sentiment, funding.funding_rate, &onchain);
    println!("\n[9] Composite signal: {:.3} ({})", signal.composite_score, signal.interpretation());
    println!("\n=== Pipeline complete ===");
}
