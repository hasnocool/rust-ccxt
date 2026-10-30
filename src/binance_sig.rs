//! Binance HMAC-SHA256 signing helper.
use hmac::{Hmac, Mac};
use sha2::Sha256;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

type HmacSha256 = Hmac<Sha256>;

pub fn sign(secret: &str, data: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(data.as_bytes());
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

pub fn build_query(params: &[(&str, &str)]) -> String {
    let mut s = String::new();
    for (i, (k, v)) in params.iter().enumerate() {
        if i > 0 { s.push('&'); }
        s.push_str(&utf8_percent_encode(k, NON_ALPHANUMERIC).to_string());
        s.push('=');
        s.push_str(&utf8_percent_encode(v, NON_ALPHANUMERIC).to_string());
    }
    s
}
