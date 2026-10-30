//! rust-ccxt: Rust port of CCXT (CryptoCurrency eXchange Trading library).
//!
//! Provides a unified async trait (`Exchange`) over 20+ crypto exchange adapters.

pub mod error;
pub mod binance_sig;
pub mod bybit_sig;
pub mod kraken_sig;
pub mod exchanges;
pub mod types;
pub mod onchain;
pub mod derivatives;
pub mod correlation;
pub mod sentiment;
pub mod cache;

pub use error::{CcxtError, CcxtResult};
pub use exchanges::*;
pub use types::*;
pub use onchain::*;
pub use derivatives::*;
pub use correlation::*;

use async_trait::async_trait;

/// Unified exchange adapter trait matching CCXT unified API.
#[async_trait]
pub trait Exchange: Send + Sync {
    /// Fetch ticker for a trading pair (e.g., "BTC/USDT").
    async fn fetch_ticker(&self, symbol: &str) -> CcxtResult<Ticker>;

    /// Fetch current order book snapshot.
    async fn fetch_order_book(
        &self,
        symbol: &str,
        limit: Option<u32>,
    ) -> CcxtResult<OrderBook>;

    /// Fetch account balance (requires auth if exchange mandates it).
    async fn fetch_balance(&self) -> CcxtResult<Balance>;

    /// Create a limit or market order.
    async fn create_order(
        &self,
        symbol: &str,
        side: OrderSide,
        amount: f64,
        price: Option<f64>,
    ) -> CcxtResult<Order>;

    /// Exchange identifier (e.g., "binance", "coinbasepro").
    fn id(&self) -> &'static str;

    /// Whether the adapter is implemented (stub vs real).
    fn is_implemented(&self) -> bool;
}
