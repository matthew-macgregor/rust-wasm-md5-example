use wasm_bindgen::prelude::*;
use md5::{Md5, Digest};
use hex;

#[wasm_bindgen]
pub fn calculate_md5(bytes: &[u8]) -> String {
    // create a Md5 hasher instance
    let mut hasher = Md5::new();

    // process input message
    hasher.update(bytes);

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 16]
    let result = hasher.finalize();
    return hex::encode(result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
