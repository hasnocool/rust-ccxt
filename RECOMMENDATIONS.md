# rust-ccxt Recommendations

Feature ideas, standalone crates, and plugin concepts for extending the rustCCXT ecosystem.

## Standalone Crates (new projects)

### 1. rustCCXT.arbitrage
Cross-exchange arbitrage scanner. Compare prices across all 56 adapters, detect triangular and spatial arbitrage opportunities, calculate fees/slippage, output ranked opportunities.

### 2. rustCCXT.backtest
Backtesting engine. Replay historical OHLCV from exchanges, run strategy signals (from correlation engine), compute PnL, Sharpe, max drawdown, win rate. Integrate with rustCCXT.stocks for equities.

### 3. rustCCXT.portfolio
Portfolio tracker. Aggregate balances across exchanges + on-chain wallets, compute allocation, rebalance suggestions, PnL tracking, tax lot management.

### 4. rustCCXT.alerts
Alert system. Price thresholds, funding rate spikes, news sentiment shifts, whale movements. Output via webhook, Discord, Telegram, email.

### 5. rustCCXT.ws
WebSocket streaming layer. Real-time ticker/orderbook/trade streams from Binance, Bybit, OKX, Kraken. Async channels for consumers.

### 6. rustCCXT.mev
MEV (Maximal Extractable Value) tools. Sandwich detection, front-running protection, mempool monitoring via mempool.space.

### 7. rustCCXT.whales
Whale tracker. Monitor large on-chain transfers, exchange cold wallet changes, smart money flows. Alert on significant movements.

### 8. rustCCXT.nft
NFT data. Floor prices, collection stats, marketplace activity (OpenSea, Blur, Magic Eden). Free public APIs.

### 9. rustCCXT.social
Social sentiment. Twitter/X crypto sentiment (via free Nitter instances or X API), Telegram/Discord message volume, Reddit (via Pushshift or .rss).

### 10. rustCCXT.macro
Macro data aggregator. FRED, BLS, SEC EDGAR, World Bank. Correlate macro releases with crypto prices.

### 11. rustCCXT.options
Options data. Deribit options chain, Greeks, IV surface, put/call ratios.

### 12. rustCCXT.lending
DeFi lending rates. Aave, Compound, MakerDAO rates. Compare CeFi vs DeFi yields.

### 13. rustCCXT.stablecoins
Stablecoin monitoring. Peg deviation, supply changes, de-peg alerts, reserve transparency.

## rustCCXT Core Enhancements

### Exchange adapters
- [x] Implement HMAC auth for all 56 providers (currently only Bybit/Kraken) — skeletons added
- [ ] WebSocket streaming for top 10 exchanges — rustCCXT.ws crate scaffolded
- [x] Rate limiter per exchange (respect API limits) — rate_limiter module
- [ ] Retry with exponential backoff
- [ ] Proxy support
- [ ] Testnet/sandbox mode

### Data & analytics
- [x] OHLCV aggregation (1m, 5m, 1h, 1d candles from trades) — ohlcv module
- [x] Technical indicators (RSI, MACD, Bollinger, EMA, SMA, ATR) — indicators module
- [ ] Volume profile / VPVR
- [ ] Order book imbalance signals
- [ ] Funding rate arbitrage detection
- [ ] Liquidation heatmap

### On-chain
- [ ] Ethereum on-chain (Etherscan, mempool)
- [ ] Solana on-chain (Solscan)
- [ ] Multi-chain support (Polygon, Arbitrum, Base, etc.)
- [ ] Whale address clustering
- [ ] Exchange flow alerts

### News & sentiment
- [ ] NLP sentiment (beyond keyword matching — use local LLM)
- [ ] News deduplication across sources
- [ ] Entity extraction (which coins mentioned)
- [ ] Impact scoring (news → price movement correlation)
- [ ] RSS feed discovery

### Correlation engine
- [ ] ML-based signal (train on historical data)
- [ ] Multi-timeframe analysis
- [ ] Cross-asset correlation (BTC vs ETH, stocks vs crypto)
- [ ] Regime detection (bull/bear/sideways)
- [ ] Risk scoring (VaR, volatility)

## Plugin System

### Plugin architecture
- Trait-based plugin system: `trait Plugin { fn name(&self) -> &str; async fn run(&self, ctx: &Context); }`
- Plugin registry with hot-reload
- Config-driven plugin enable/disable
- Plugin marketplace (git-based)

### Example plugins
- **Telegram bot**: price alerts, portfolio updates, news digest
- **Discord bot**: slash commands for price, news, sentiment
- **Webhook notifier**: POST signals to user-defined URLs
- **CSV exporter**: dump trades/balances to CSV
- **Grafana exporter**: Prometheus metrics for dashboards
- **Slack notifier**: channel alerts
- **Email digest**: daily summary email
- **Google Sheets sync**: portfolio to spreadsheet
- **Home Assistant**: crypto price sensors

## Infrastructure

### Caching
- [x] In-memory TTL cache (done)
- [ ] Redis backend for distributed caching
- [ ] Disk cache for historical data
- [ ] Cache invalidation strategies

### Storage
- [ ] SQLite for local data (trades, balances, history)
- [ ] PostgreSQL for production
- [ ] Time-series DB (InfluxDB/TimescaleDB) for OHLCV

### Deployment
- [ ] Docker image
- [ ] Kubernetes helm chart
- [ ] Systemd service
- [ ] Cloudflare Workers (WASM) for edge

### Developer experience
- [ ] CLI improvements (interactive mode, TUI)
- [ ] Web dashboard (Axum/Actix + Vue/React)
- [ ] Python bindings (PyO3)
- [ ] Node.js bindings (napi-rs)
- [ ] WASM for browser

## Free Data Sources to Integrate

| Source | Data | Auth |
|---|---|---|
| CoinGecko | Prices, markets, trending | None (free tier) |
| DexScreener | DEX pairs, liquidity | None |
| GeckoTerminal | Pool data, OHLCV | None |
| mempool.space | Bitcoin mempool, fees | None |
| blockchain.info | Hashrate, stats | None |
| Binance public | Ticker, orderbook, funding | None |
| Bybit public | Ticker, orderbook, funding | None |
| CoinCap | Prices, markets | None |
| CryptoCompare | Prices, news | Free key |
| Messari | Asset profiles, metrics | Free key |
| DeFi Llama | TVL, yields, protocols | None |
| Glassnode | On-chain (limited free) | Free key |
| Santiment | Social, on-chain | Free key |
| LunarCrush | Social sentiment | Free key |
| Alternative.me | Fear & Greed index | None |
| Google Trends | Search interest | None (unofficial) |
| Reddit .rss | Subreddit feeds | None (blocked sometimes) |
| Hacker News | Tech news | None |
| Google News RSS | News search | None |
| NewsAPI | News articles | Free key |
| Alpha Vantage | Stocks, forex | Free key |
| FRED | Macro data | Free key |
| Yahoo Finance | Stocks (unofficial) | None (rate limited) |

## Priority Order (recommended)

1. **WebSocket streaming** (rustCCXT.ws) — real-time data is highest value
2. **HMAC auth for top 10 exchanges** — enables actual trading
3. **Sentiment NLP upgrade** — better signals than keyword matching
4. **Backtesting engine** — validate strategies before live
5. **Alert system** — actionable notifications
6. **Portfolio tracker** — user value
7. **Plugin system** — extensibility
8. **Python/Node bindings** — broader adoption

## New High-Impact Ideas (Free-Data-First)

### A. New Standalone Crates
1. **rustCCXT.liquidations** — liquidation heatmaps from Hyperliquid/Binance liquidation feeds (public), detect cascading risk
2. **rustCCXT.funding** — funding rate arbitrage monitor across perpetuals, daily funding delta
3. **rustCCXT.gas** — multi-chain gas tracker (EVM RPC free endpoints, Solana, Bitcoin fees)
4. **rustCCXT.discord/twitter/rss** — lightweight social ingest via free RSS/Nitter, sentiment per coin
5. **rustCCXT.flash** — flash crash detector (price drop > X% in Y seconds), webhook alerts
6. **rustCCXT.tax** — tax lot tracking, realized/unrealized PnL, CSV export for taxes
7. **rustCCXT.ema** — email alerts crate (SMTP), Telegram/Discord webhook plugins
8. **rustCCXT.ai** — local LLM sentiment summarizer (llama.cpp), no cloud API

### B. Core Enhancements
- Unified rate limiter per IP + global (token bucket with Redis)
- Exponential backoff + jitter for all HTTP clients
- Order-book reconstruction from trades (LOB simulation)
- Multi-timeframe indicators (M1/H1/D1 correlation)
- Cross-exchange symbol mapping (BTC-USDT vs BTC/USD normalization)
- Testnet sandboxes for Bybit/Binance (isolated auth)
- WASM build for browser playground

### C. Additional Free Data Sources
| Source | Data |
|---|---|
| DeFiLlama | TVL, yields |
| Flipside Crypto | SQL queries |
| Dune Analytics | Public dashboards |
| CoinGecko | Market cap, volumes |
| CoinMarketCap free | Top 100 prices |
| Binance OpenAPI | Full market data |
| OKX public | Funding, depth |
| Kraken public | OHLCV |
| CryptoQuant (free) | Basic metrics |
| Glassnode Studio (free) | Daily stats |
