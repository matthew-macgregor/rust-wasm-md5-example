use wasm_bindgen::prelude::*;
use md5::{Md5, Digest};
use hex;
use base64;

fn calculate_md5_bytes(bytes: &[u8]) -> [u8; 16] {
    // create a Md5 hasher instance
    let mut hasher = Md5::new();

    // process input message
    hasher.update(bytes);

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 16]
    return hasher.finalize().into();
}

pub const BASE_64: &'static str = "base64";
pub const HEX: &'static str = "hex";

#[wasm_bindgen]
pub fn calculate_md5(bytes: &[u8], encoding: &str) -> String {
    match encoding {
        BASE_64     => return calculate_md5_base64(bytes),
        HEX         => return calculate_md5_hex(bytes),
        _           => return "".to_string()
    }
}

#[wasm_bindgen]
pub fn calculate_md5_base64(bytes: &[u8]) -> String {
    let hash: [u8; 16] = calculate_md5_bytes(bytes);
    return base64::encode(hash);
}

#[wasm_bindgen]
pub fn calculate_md5_hex(bytes: &[u8]) -> String {
    let hash: [u8; 16] = calculate_md5_bytes(bytes);
    return hex::encode(hash);
}

#[wasm_bindgen]
pub fn hex_to_base64(encoded: &str) -> String {
    let decoded = hex::decode(encoded).unwrap();
    return base64::encode(decoded);
}

#[wasm_bindgen]
pub fn base64_to_hex(encoded: &str) -> String {
    let decoded = base64::decode(encoded).unwrap();
    return hex::encode(decoded);
}

#[cfg(test)]
mod tests {
    #![cfg(target_arch = "wasm32")]

    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    use crate::{hex_to_base64, base64_to_hex};

    #[wasm_bindgen_test]
    fn test_hex_base64_conversion() {
        let b64 = hex_to_base64(&"FC3FF98E8C6A0D3087D515C0473F8677".to_ascii_lowercase());
        assert_eq!("/D/5joxqDTCH1RXARz+Gdw==", b64);
        let hex = base64_to_hex(&b64);
        assert_eq!("FC3FF98E8C6A0D3087D515C0473F8677".to_ascii_lowercase(), hex);
    }
}