use crate::Bytes;
use openssl::symm::{decrypt, Cipher};

pub fn aes_ecb(input: &Bytes, key: &Bytes) -> Bytes {
  let cipher = Cipher::aes_128_ecb();
  let result = decrypt(cipher, key, None, input);
  match result {
    Ok(bytes) => { bytes },
    Err(_) => { Bytes::new() }
  }
}
