use clap::{Parser, Subcommand};
use rust_ccxt::{
    Exchange, cache,
    onchain::Onchain, derivatives::Derivatives, correlation::CorrelationEngine,
    dexscreener::DexScreener, coingecko::CoinGecko, geckoterminal::GeckoTerminal,
    news, sentiment::Sentiment, stocks,
    exchanges::binance::BinanceExchange,
};

#[derive(Parser)]
#[command(name = "rust-ccxt", about = "Crypto + news + stocks + on-chain CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run full end-to-end pipeline
    Pipeline,
    /// Fetch exchange ticker
    Ticker { symbol: String },
    /// On-chain snapshot
    Onchain,
    /// Derivatives funding + OI
    Derivatives { symbol: String },
    /// DexScreener search
    Dex { query: String },
    /// CoinGecko price
    Coin { id: String },
    /// GeckoTerminal top pools
    Pools { network: String },
    /// News + sentiment
    News { query: String },
    /// Stock quote (Alpha Vantage)
    Stock { symbol: String },
    /// Clear cache
    CacheClear,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Pipeline => run_pipeline().await,
        Commands::Ticker { symbol } => {
            let ex = BinanceExchange::new();
            match ex.fetch_ticker(&symbol).await {
                Ok(t) => println!("{}: ${:.2} (24h vol: {})", t.symbol, t.last, t.volume),
                Err(e) => println!("err: {}", e),
            }
        }
        Commands::Onchain => {
            let s = Onchain::snapshot().await.unwrap_or_else(|_| rust_ccxt::onchain::OnchainSnapshot {
                btc_price_usd: 0.0, hashrate_ehs: 0.0, mempool_tx_count: 0, mempool_vsize: 0,
                fee_fast_sat_vb: 0.0, fee_medium_sat_vb: 0.0, fee_slow_sat_vb: 0.0, block_height: 0,
            });
            println!("BTC: ${:.2}", s.btc_price_usd);
            println!("Hashrate: {:.2} EH/s", s.hashrate_ehs);
            println!("Mempool: {} tx, {} vbytes", s.mempool_tx_count, s.mempool_vsize);
            println!("Fees: fast {:.1} / med {:.1} / slow {:.1} sat/vB", s.fee_fast_sat_vb, s.fee_medium_sat_vb, s.fee_slow_sat_vb);
            println!("Block height: {}", s.block_height);
        }
        Commands::Derivatives { symbol } => {
            let f = Derivatives::binance_funding(&symbol).await.unwrap_or_else(|_| rust_ccxt::derivatives::FundingRate {
                symbol: symbol.clone(), funding_rate: 0.0, funding_time: 0,
            });
            let oi = Derivatives::binance_open_interest(&symbol).await.unwrap_or_else(|_| rust_ccxt::derivatives::OpenInterest {
                symbol: symbol.clone(), open_interest: 0.0, timestamp: 0,
            });
            println!("{} funding: {:.6}", f.symbol, f.funding_rate);
            println!("{} open interest: {}", oi.symbol, oi.open_interest);
        }
        Commands::Dex { query } => {
            match DexScreener::search(&query).await {
                Ok(pairs) => {
                    println!("DexScreener '{}': {} pairs", query, pairs.len());
                    for p in pairs.iter().take(5) {
                        println!("  {} on {} - ${:.6} (24h: {:+.2}%)", p.base_token, p.chain_id, p.price_usd, p.price_change_24h);
                    }
                }
                Err(e) => println!("err: {}", e),
            }
        }
        Commands::Coin { id } => {
            match CoinGecko::price(&id, "usd").await {
                Ok(v) => println!("{}: {}", id, v[&id]["usd"]),
                Err(e) => println!("err: {}", e),
            }
        }
        Commands::Pools { network } => {
            match GeckoTerminal::top_pools(&network, 1).await {
                Ok(pools) => {
                    println!("GeckoTerminal {} pools: {}", network, pools.len());
                    for p in pools.iter().take(5) {
                        println!("  {} - vol24h: ${:.0}", p.name, p.volume_24h);
                    }
                }
                Err(e) => println!("err: {}", e),
            }
        }
        Commands::News { query } => {
            let items = news::NewsClient::google_news(&query).await.unwrap_or_default();
            let sentiment = Sentiment::average_items(&items);
            println!("News '{}': {} items, sentiment {:.3}", query, items.len(), sentiment);
            for it in items.iter().take(5) {
                println!("  - {}", it.title);
            }
        }
        Commands::Stock { symbol } => {
            let av = stocks::StockClient::alpha_vantage("0A32OAYKCCTH3RPF");
            match av.quote(&symbol).await {
                Ok(q) => println!("{}: ${:.2} ({:+.2}%)", q.symbol, q.price, q.change_percent),
                Err(e) => println!("err: {}", e),
            }
        }
        Commands::CacheClear => {
            cache::clear();
            println!("cache cleared");
        }
    }
}

async fn run_pipeline() {
    println!("=== rustCCXT Pipeline ===\n");
    let ex = BinanceExchange::new();
    let price = ex.fetch_ticker("BTCUSDT").await.map(|t| t.last).unwrap_or(0.0);
    println!("[1] Binance BTCUSDT: ${:.2}", price);

    let onchain = Onchain::snapshot().await.unwrap_or_else(|_| rust_ccxt::onchain::OnchainSnapshot {
        btc_price_usd: 0.0, hashrate_ehs: 0.0, mempool_tx_count: 0, mempool_vsize: 0,
        fee_fast_sat_vb: 0.0, fee_medium_sat_vb: 0.0, fee_slow_sat_vb: 0.0, block_height: 0,
    });
    println!("[2] On-chain: BTC ${:.2}, mempool {} tx", onchain.btc_price_usd, onchain.mempool_tx_count);

    let funding = Derivatives::binance_funding("BTCUSDT").await.unwrap_or_else(|_| rust_ccxt::derivatives::FundingRate {
        symbol: "BTCUSDT".into(), funding_rate: 0.0, funding_time: 0,
    });
    println!("[3] Funding: {:.6}", funding.funding_rate);

    if let Ok(pairs) = DexScreener::search("BTC").await {
        println!("[4] DexScreener: {} BTC pairs", pairs.len());
    }
    if let Ok(v) = CoinGecko::price("bitcoin", "usd").await {
        println!("[5] CoinGecko BTC: ${}", v["bitcoin"]["usd"]);
    }
    if let Ok(pools) = GeckoTerminal::top_pools("eth", 1).await {
        println!("[6] GeckoTerminal: {} pools", pools.len());
    }

    let news_items = news::NewsClient::google_news("bitcoin").await.unwrap_or_default();
    let sentiment = Sentiment::average_items(&news_items);
    println!("[7] News: {} items, sentiment {:.3}", news_items.len(), sentiment);

    let av = stocks::StockClient::alpha_vantage("0A32OAYKCCTH3RPF");
    if let Ok(q) = av.quote("AAPL").await {
        println!("[8] AAPL: ${:.2}", q.price);
    }

    let signal = CorrelationEngine::analyze(2.5, sentiment, funding.funding_rate, &onchain);
    println!("\n[9] Composite: {:.3} ({})", signal.composite_score, signal.interpretation());
}

