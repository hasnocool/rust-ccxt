use rust_ccxt::rust_ccxt::MacroClient;
#[tokio::main]
async fn main() {
    let m = MacroClient::new("d292b936f214d28eff85bab336c5b828");
    let v = m.fred_latest("DGS10").await.unwrap();
    println!("FRED 10Y Treasury: {}", v);
}
