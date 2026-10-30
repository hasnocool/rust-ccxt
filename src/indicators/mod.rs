//! Technical indicators operating on OHLCV bars.

use rust_ccxt_stocks::StockBar;

#[derive(Debug, Clone)]
pub struct Indicators;

impl Indicators {
    /// Simple Moving Average.
    pub fn sma(values: &[f64], period: usize) -> Vec<f64> {
        if values.len() < period { return vec![]; }
        let mut out = vec![0.0; values.len()];
        let mut sum: f64 = values[..period].iter().sum();
        out[period - 1] = sum / period as f64;
        for i in period..values.len() {
            sum += values[i] - values[i - period];
            out[i] = sum / period as f64;
        }
        out[period - 1..].to_vec()
    }

    /// Exponential Moving Average.
    pub fn ema(values: &[f64], period: usize) -> Vec<f64> {
        if values.is_empty() { return vec![]; }
        let k = 2.0 / (period as f64 + 1.0);
        let mut out = vec![0.0; values.len()];
        out[0] = values[0];
        for i in 1..values.len() {
            out[i] = values[i] * k + out[i - 1] * (1.0 - k);
        }
        out
    }

    /// RSI (Relative Strength Index).
    pub fn rsi(values: &[f64], period: usize) -> Vec<f64> {
        if values.len() < period + 1 { return vec![]; }
        let mut gains = vec![0.0; values.len()];
        let mut losses = vec![0.0; values.len()];
        for i in 1..values.len() {
            let diff = values[i] - values[i - 1];
            if diff > 0.0 { gains[i] = diff; } else { losses[i] = -diff; }
        }
        let mut avg_gain: f64 = gains[1..=period].iter().sum::<f64>() / period as f64;
        let mut avg_loss: f64 = losses[1..=period].iter().sum::<f64>() / period as f64;
        let mut out = vec![50.0; values.len() - period];
        let rs = if avg_loss == 0.0 { f64::INFINITY } else { avg_gain / avg_loss };
        out[0] = 100.0 - (100.0 / (1.0 + rs));
        for i in (period + 1)..values.len() {
            avg_gain = (avg_gain * (period as f64 - 1.0) + gains[i]) / period as f64;
            avg_loss = (avg_loss * (period as f64 - 1.0) + losses[i]) / period as f64;
            let rs = if avg_loss == 0.0 { f64::INFINITY } else { avg_gain / avg_loss };
            out[i - period] = 100.0 - (100.0 / (1.0 + rs));
        }
        out
    }

    /// MACD (Moving Average Convergence Divergence).
    pub fn macd(values: &[f64], fast: usize, slow: usize, signal: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let ema_fast = Self::ema(values, fast);
        let ema_slow = Self::ema(values, slow);
        let macd_line: Vec<f64> = ema_fast.iter().zip(ema_slow.iter()).map(|(a, b)| a - b).collect();
        let signal_line = Self::ema(&macd_line, signal);
        let histogram: Vec<f64> = macd_line.iter().zip(signal_line.iter()).map(|(a, b)| a - b).collect();
        (macd_line, signal_line, histogram)
    }

    /// Bollinger Bands (upper, middle, lower).
    pub fn bollinger(values: &[f64], period: usize, mult: f64) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let sma = Self::sma(values, period);
        let mut upper = vec![0.0; sma.len()];
        let mut lower = vec![0.0; sma.len()];
        let offset = values.len() - sma.len();
        for (i, &m) in sma.iter().enumerate() {
            let idx = i + offset;
            let slice = &values[idx.saturating_sub(period - 1)..=idx];
            let var: f64 = slice.iter().map(|x| (x - m).powi(2)).sum::<f64>() / slice.len() as f64;
            let std = var.sqrt();
            upper[i] = m + mult * std;
            lower[i] = m - mult * std;
        }
        (upper, sma, lower)
    }

    /// ATR (Average True Range).
    pub fn atr(bars: &[StockBar], period: usize) -> Vec<f64> {
        if bars.len() < 2 { return vec![]; }
        let mut trs = vec![0.0; bars.len()];
        trs[0] = bars[0].high - bars[0].low;
        for i in 1..bars.len() {
            let hl = bars[i].high - bars[i].low;
            let hc = (bars[i].high - bars[i - 1].close).abs();
            let lc = (bars[i].low - bars[i - 1].close).abs();
            trs[i] = hl.max(hc).max(lc);
        }
        Self::sma(&trs, period)
    }
}
