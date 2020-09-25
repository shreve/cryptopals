use super::Bytes;
use std::{char, u8};

pub fn hex_to_bytes(input: &str) -> Bytes {
    input
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .collect::<Bytes>()
        .chunks(2)
        .map(|pair| pair[0] << 4 | pair[1])
        .collect()
}

pub fn bytes_to_hex(input: Bytes) -> String {
    let mut out = String::new();
    for byte in input.iter() {
        out.push(char::from_digit((byte >> 4) as u32, 16).unwrap());
        out.push(char::from_digit((byte & 0xF) as u32, 16).unwrap());
    }
    out
}

//
// Encode bytes as base64
//
// bytes: 0 0 0 0 0 0 0 0|1 1 1 1 1 1 1 1|2 2 2 2 2 2 2 2
// b64:   0 0 0 0 0 0|1 1 1 1 1 1|2 2 2 2 2 2|3 3 3 3 3 3
//
pub fn bytes_to_b64(input: Bytes) -> String {
    // Collect the base64 alphabet as bytes
    let b64: Vec<u8> = (0x41..0x5B)
        .chain(0x61..0x7B)
        .chain(0x30..0x3A)
        .chain(0x2B..0x2C)
        .chain(0x2F..0x30)
        .collect();

    let mut out: Bytes =
        vec![0; (input.len() as f64 * 4.0 / 3.0).ceil() as usize];

    // Use separate index j to keep track of output byte index
    let mut j = 0;
    for (i, byte) in input.iter().enumerate() {
        match i % 3 {
            0 => {
                out[j] = (byte & 0xFC) >> 2;
                out[j + 1] = (byte & 0x3) << 4;
            }
            1 => {
                out[j] |= (byte & 0xF0) >> 4;
                out[j + 1] = (byte & 0xF) << 2;
            }
            2 => {
                out[j] |= (byte & 0xC0) >> 6;
                out[j + 1] = byte & 0x3F;
                j = j + 1;
            }
            _ => {}
        }
        j = j + 1;
    }

    out.iter().map(|ind| b64[*ind as usize] as char).collect()
}

pub fn b64_to_bytes(input: &str) -> Bytes {
    assert!(input.len() % 4 == 0);
    let b64: Vec<u8> = (0x41..0x5B)
        .chain(0x61..0x7B)
        .chain(0x30..0x3A)
        .chain(0x2B..0x2C)
        .chain(0x2F..0x30)
        .collect();

    let mut out = Bytes::new();
    let sanitized = input.replace("\n", "");

    let mut buf: u32 = 0;
    let mut remshift = 0;
    for (i, chr) in sanitized.chars().enumerate() {
        if chr == '=' {
            break;
        }

        let index = b64.iter().position(|b| *b == chr as u8);
        let val = match index {
            Some(i) => i,
            None => continue,
        };

        buf <<= 6;
        buf |= val as u32 & 0x3F;
        remshift -= 1;

        if i % 4 == 3 {
            out.push((buf >> 16) as u8 & 0xFF);
            out.push((buf >> 8) as u8 & 0xFF);
            out.push(buf as u8 & 0xFF);
            buf = 0;
            remshift = 4;
        }
    }

    buf <<= 6 * remshift;

    if (buf >> 16) as u8 > 0 {
        out.push((buf >> 16) as u8);
    }

    if (buf >> 8) as u8 > 0 {
        out.push((buf >> 8) as u8);
    }

    if buf as u8 > 0 {
        out.push(buf as u8);
    }

    out
}

pub fn hex_to_b64(input: &str) -> String {
    bytes_to_b64(hex_to_bytes(input))
}

#[cfg(test)]
mod test {

    #[test]
    fn test_b64_to_bytes() {
        assert_eq!(
            super::b64_to_bytes("dGhpcyBpcyBhIHRlc3Q="),
            "this is a test".as_bytes()
        );

        assert_eq!(
            super::b64_to_bytes("dGhpcyBpcyBhIHRlc3QhIQ=="),
            "this is a test!!".as_bytes()
        );
    }
}
