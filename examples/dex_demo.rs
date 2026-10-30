use rust_ccxt::{DexScreener, CoinGecko, GeckoTerminal};

#[tokio::main]
async fn main() {
    // DexScreener
    match DexScreener::search("PEPE").await {
        Ok(pairs) => {
            println!("DexScreener PEPE pairs: {}", pairs.len());
            for p in pairs.iter().take(3) {
                println!("  {} on {} - ${:.6} (24h: {:+.2}%)", p.base_token, p.chain_id, p.price_usd, p.price_change_24h);
            }
        }
        Err(e) => println!("DexScreener error: {}", e),
    }

    // CoinGecko markets
    match CoinGecko::markets("usd", 5, 1).await {
        Ok(coins) => {
            println!("\nCoinGecko top 5:");
            for c in &coins {
                println!("  {} ({}): ${:.2} ({:+.2}%)", c.name, c.symbol, c.price_usd, c.price_change_24h);
            }
        }
        Err(e) => println!("CoinGecko error: {}", e),
    }

    // CoinGecko trending
    match CoinGecko::trending().await {
        Ok(v) => {
            if let Some(coins) = v["coins"].as_array() {
                println!("\nCoinGecko trending: {}", coins.len());
            }
        }
        Err(e) => println!("trending error: {}", e),
    }

    // GeckoTerminal top pools on ethereum
    match GeckoTerminal::top_pools("eth", 1).await {
        Ok(pools) => {
            println!("\nGeckoTerminal ETH top pools: {}", pools.len());
            for p in pools.iter().take(3) {
                println!("  {} - vol24h: ${:.0}", p.name, p.volume_24h);
            }
        }
        Err(e) => println!("GeckoTerminal error: {}", e),
    }
}
