//
// Set 2: Block Crypto
//
// https://cryptopals.com/sets/2
//

#[cfg(test)]
use crate::*;

#[test]
fn test_challenge_9() {
  let input = "YELLOW SUBMARINE".as_bytes().to_vec();
  assert_eq!(
    padding::pkcs7(&input, 20),
    "YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes()
  )
}
