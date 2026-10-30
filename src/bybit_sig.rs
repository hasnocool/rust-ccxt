use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;

type HmacSha256 = Hmac<Sha256>;

pub fn sign(secret: &str, params: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("valid key");
    mac.update(params.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}
