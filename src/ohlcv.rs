//! OHLCV aggregation from raw trades or bars.

use crate::types::Ticker;

#[derive(Debug, Clone)]
pub struct OhlcvBar {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

pub struct Ohlcv;

impl Ohlcv {
    /// Aggregate tickers into bars of `period_secs` duration.
    pub fn aggregate(ticks: &[(i64, f64, f64)], period_secs: i64) -> Vec<OhlcvBar> {
        // ticks: (timestamp, price, volume)
        if ticks.is_empty() { return vec![]; }
        let mut bars = vec![];
        let mut cur_start = ticks[0].0 - (ticks[0].0 % period_secs);
        let mut o = ticks[0].1;
        let mut h = ticks[0].1;
        let mut l = ticks[0].1;
        let mut c = ticks[0].1;
        let mut v = ticks[0].2;
        for (ts, price, vol) in ticks.iter().skip(1) {
            let bucket = ts - (ts % period_secs);
            if bucket != cur_start {
                bars.push(OhlcvBar { timestamp: cur_start, open: o, high: h, low: l, close: c, volume: v });
                cur_start = bucket;
                o = *price; h = *price; l = *price; c = *price; v = *vol;
            } else {
                h = h.max(*price);
                l = l.min(*price);
                c = *price;
                v += vol;
            }
        }
        bars.push(OhlcvBar { timestamp: cur_start, open: o, high: h, low: l, close: c, volume: v });
        bars
    }
}
