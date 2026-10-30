use rust_ccxt::{news, sentiment, onchain, derivatives, correlation, dexscreener, coingecko, geckoterminal};
#[tokio::main]
async fn main() {
    println!("sentiment: {}", rust_ccxt::sentiment::Sentiment::score_text("bitcoin rally surge"));
    let items = news::NewsClient::google_news("bitcoin").await.unwrap_or_default();
    println!("news items: {}", items.len());
    let s = sentiment::Sentiment::average_items(&items);
    println!("news sentiment: {:.3}", s);
    println!("modules loaded OK");
}
