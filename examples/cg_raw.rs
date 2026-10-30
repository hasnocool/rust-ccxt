use reqwest;
#[tokio::main]
async fn main() {
    let url = "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&order=market_cap_desc&per_page=3&page=1&sparkline=false";
    let resp = reqwest::get(url).await.unwrap();
    println!("status: {}", resp.status());
    let text = resp.text().await.unwrap();
    println!("body: {}", &text[..text.len().min(300)]);
}
