//! Correlation engine: combine price, news sentiment, on-chain, derivatives.

use crate::onchain::OnchainSnapshot;
use crate::derivatives::FundingRate;

#[derive(Debug, Clone)]
pub struct MarketSignal {
    pub price_change_pct: f64,
    pub news_sentiment: f64,   // -1.0 .. 1.0
    pub funding_rate: f64,
    pub mempool_pressure: f64, // 0..1
    pub composite_score: f64,  // -1.0 .. 1.0
}

impl MarketSignal {
    /// Build a composite signal from components.
    /// Weights: price 0.3, sentiment 0.3, funding 0.2, mempool 0.2
    pub fn from_components(
        price_change_pct: f64,
        news_sentiment: f64,
        funding_rate: f64,
        onchain: &OnchainSnapshot,
    ) -> Self {
        // Normalize price change to [-1,1] via tanh
        let price_norm = (price_change_pct / 5.0).tanh();
        // Funding: positive funding = longs pay shorts (bullish crowded) -> negative signal
        let funding_norm = (-funding_rate * 100.0).tanh();
        // Mempool pressure: high vsize -> congestion -> 0..1
        let mempool_pressure = (onchain.mempool_vsize as f64 / 50_000_000.0).min(1.0);
        let mempool_signal = -mempool_pressure * 0.5; // congestion slightly bearish
        let composite = 0.3 * price_norm
            + 0.3 * news_sentiment
            + 0.2 * funding_norm
            + 0.2 * mempool_signal;
        let composite = composite.clamp(-1.0, 1.0);
        Self {
            price_change_pct,
            news_sentiment,
            funding_rate,
            mempool_pressure,
            composite_score: composite,
        }
    }

    pub fn interpretation(&self) -> &'static str {
        if self.composite_score > 0.5 {
            "strongly bullish"
        } else if self.composite_score > 0.15 {
            "bullish"
        } else if self.composite_score < -0.5 {
            "strongly bearish"
        } else if self.composite_score < -0.15 {
            "bearish"
        } else {
            "neutral"
        }
    }
}

pub struct CorrelationEngine;

impl CorrelationEngine {
    /// Quick analysis given price change, sentiment, and on-chain snapshot.
    pub fn analyze(
        price_change_pct: f64,
        news_sentiment: f64,
        funding_rate: f64,
        onchain: &OnchainSnapshot,
    ) -> MarketSignal {
        MarketSignal::from_components(price_change_pct, news_sentiment, funding_rate, onchain)
    }
}
