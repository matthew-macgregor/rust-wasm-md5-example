use md5::{Md5, Digest};
use hex_literal::hex;

fn main() {
    // create a Md5 hasher instance
    let mut hasher = Md5::new();

    // process input message
    hasher.update(b"hello world");

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 16]
    let result = hasher.finalize();
    assert_eq!(result[..], hex!("5eb63bbbe01eeed093cb22bb8f5acdc3"));
}
