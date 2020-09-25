use super::Bytes;

pub fn repeating_key_cipher(input: &Bytes, key: &Bytes) -> Bytes {
    let keylen = key.len();
    let msglen = input.len();
    let mut out = Bytes::new();

    for i in 0..msglen {
        let j = i % keylen;

        out.push(input[i] ^ key[j]);
    }

    out
}
