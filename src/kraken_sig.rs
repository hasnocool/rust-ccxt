use hmac::{Hmac, Mac};
use sha2::Sha512;
use base64::{engine::general_purpose, Engine as _};

type HmacSha512 = Hmac<Sha512>;

pub fn sign(secret: &str, message: &[u8]) -> String {
    let mut mac = HmacSha512::new_from_slice(secret.as_bytes()).expect("valid key");
    mac.update(message);
    general_purpose::STANDARD.encode(mac.finalize().into_bytes())
}
