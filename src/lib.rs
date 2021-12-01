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
    use crate::{hex_to_base64, base64_to_hex, calculate_md5_hex, calculate_md5_base64, calculate_md5};
    use crate::{BASE_64, HEX};

    #[wasm_bindgen_test]
    fn test_calculate_md5() {
        let b64 = calculate_md5(b"123abc", BASE_64);
        assert_eq!("qQZEnVdp+nNh1+zGqj9tKA==", b64);
        let hex = calculate_md5(b"123abc", HEX);
        assert_eq!("a906449d5769fa7361d7ecc6aa3f6d28", hex);
    }

    #[wasm_bindgen_test]
    fn test_hex_to_base64() {
        let b64 = hex_to_base64("a906449d5769fa7361d7ecc6aa3f6d28");
        assert_eq!("qQZEnVdp+nNh1+zGqj9tKA==", b64);
    }

    #[wasm_bindgen_test]
    fn test_base64_to_hex() {
        let hex = base64_to_hex("qQZEnVdp+nNh1+zGqj9tKA==");
        assert_eq!("a906449d5769fa7361d7ecc6aa3f6d28", hex);
    }
    
    #[wasm_bindgen_test]
    fn test_calculate_md5_hex() {
        let checksum_hex = calculate_md5_hex(b"123abc");
        assert_eq!("a906449d5769fa7361d7ecc6aa3f6d28", checksum_hex);
    }

    #[wasm_bindgen_test]
    fn test_calculate_b64_hex() {
        let b64 = calculate_md5_base64(b"123abc");
        assert_eq!("qQZEnVdp+nNh1+zGqj9tKA==", b64);
    }
}