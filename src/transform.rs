use crate::Bytes;
use crate::encode::{bytes_to_hex,  hex_to_bytes};

pub fn byte_xor(a: &Bytes, b: &Bytes) -> Bytes {
  a.iter().zip(b).map(|(a,b)| (a ^ b)).collect()
}

pub fn char_xor(a: &Bytes, b: u8) -> Bytes {
  a.iter().map(|byte| *byte ^ b).collect()
}

pub fn hex_xor(left: &str, right: &str) -> String {
  bytes_to_hex(
    byte_xor(
      &hex_to_bytes(left),
      &hex_to_bytes(right)
    )
  )
}
