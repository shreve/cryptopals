mod cryptopals {
    type Bytes = Vec<u8>;

    pub mod transform {
        use super::Bytes;

        pub fn byte_xor(a: &Bytes, b: &Bytes) -> Bytes {
            a.iter().zip(b).map(|(a,b)| (a ^ b)).collect()
        }

        pub fn char_xor(a: &Bytes, b: u8) -> Bytes {
            a.iter().map(|byte| *byte ^ b).collect()
        }

        pub fn hex_xor(left: &str, right: &str) -> String {
            super::encode::bytes_to_hex(
                byte_xor(
                    &super::encode::hex_to_bytes(left),
                    &super::encode::hex_to_bytes(right)
                )
            )
        }
    }

    pub mod encrypt {
        use super::Bytes;

        pub fn repeating_key_cipher(input: &Bytes, key: &str) -> Bytes {
            let bkey = key.as_bytes();
            let keylen = key.len();
            let msglen = input.len();
            let mut out = Vec::new();

            for i in 0..msglen {
                let j = i % keylen;

                out.push(input[i] ^ bkey[j]);
            }

            out
        }
    }

    pub mod cracking {
        use super::Bytes;

        pub fn single_byte_cipher(input: &Bytes) -> (usize, Bytes) {
            let dict: Bytes = (0x00..0xFF).collect();

            let mut high_score = 0;
            let mut best: Bytes = vec![0; input.len()];

            for chr in dict.iter() {
                let output = super::transform::char_xor(input, *chr);
                let score = english_score(&output);

                if score > high_score {
                    high_score = score;
                    best = output;
                }
            }

            (high_score, best)
        }

        const FREQ: &str = "etaoin shrdlucmfwypvbgkjqxz";

        fn english_score(input: &Bytes) -> usize {
            let mut score = 0;

            for chr in input.iter() {
                match FREQ.chars().position(|c| c == *chr as char) {
                    Some(i) => score += 30 - i,
                    None => {}
                }
            }

            score
        }

        fn hamming_distance(left: &Bytes, right: &Bytes) -> usize {
            let mut dist: usize = 0;

            for i in 0..left.len() {
                let lbyte = left[i];
                let rbyte = right[i];
                let mut inter = lbyte ^ rbyte;

                while inter > 0 {
                    if (inter & 1) == 1 {
                        dist += 1;
                    }
                    inter >>= 1;
                }
            }

            dist
        }

        #[cfg(test)]
        mod tests {

            #[test]
            fn test_english_score() {
                assert_eq!(
                    super::english_score(&"hello, world!".as_bytes().to_vec()),
                    242
                )
            }

            #[test]
            fn test_hamming_distance() {
                assert_eq!(
                    super::hamming_distance(
                        &"this is a test".as_bytes().to_vec(),
                        &"wokka wokka!!!".as_bytes().to_vec()
                    ),
                    37
                )
            }
        }
    }

    pub mod encode {
        use std::{u8, char};
        use super::Bytes;

        pub fn hex_to_bytes(input: &str) -> Bytes {
            input
                .chars()
                .map(|c| c.to_digit(16).unwrap() as u8)
                .collect::<Bytes>()
                .chunks(2)
                .map(|pair| { pair[0] << 4 | pair[1] })
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
            let b64: Vec<u8> =
                (0x41..0x5B)
                .chain(0x61..0x7B)
                .chain(0x30..0x3A)
                .chain(0x2B..0x2C)
                .chain(0x2F..0x30)
                .collect();

            let mut out: Bytes = vec![0; (input.len() as f64 * 4.0 / 3.0).ceil() as usize];

            // Use separate index j to keep track of output byte index
            let mut j = 0;
            for (i, byte) in input.iter().enumerate() {
                match i % 3 {
                    0 => {
                        out[j] = (byte & 0xFC) >> 2;
                        out[j+1] = (byte & 0x3) << 4;
                    }
                    1 => {
                        out[j] |= (byte & 0xF0) >> 4;
                        out[j+1] = (byte & 0xF) << 2;
                    }
                    2 => {
                        out[j] |= (byte & 0xC0) >> 6;
                        out[j+1] = byte & 0x3F;
                        j = j + 1;
                    }
                    _ => { }
                }
                j = j + 1;
            }

            out.iter().map(|ind| b64[*ind as usize] as char).collect()
        }

        pub fn b64_to_bytes(input: &str) -> Bytes {
            let mut b64 =
                (0x41..0x5B)
                .chain(0x61..0x7B)
                .chain(0x30..0x3A)
                .chain(0x2B..0x2C)
                .chain(0x2F..0x30);

            let mut out = Bytes::new();
            let sanitized = input.replace("\n", "");

            let mut buf: u32 = 0;
            for (i, chr) in sanitized.chars().enumerate() {
                let val = match  b64.position(|b| b == chr as u8) {
                    Some(i) => { i },
                    None => {
                        println!("{}", chr);
                        continue
                    }
                };
                println!("{}", val);

                buf <<= 6;
                buf |= val as u32 & 0x3F;

                if i % 4 == 3 {
                    println!("{:?}", buf);
                    out.push((buf >> 16) as u8 & 0xFF);
                    out.push((buf >> 8) as u8 & 0xFF);
                    out.push(buf as u8 & 0xFF);
                    buf = 0;
                }
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
                    super::b64_to_bytes("dGhpcyBpcyBhIHRlc3Q"),
                    "this is a test".as_bytes()
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_1() {
        assert_eq!(
            cryptopals::encode::hex_to_b64(
                "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
            ),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        )
    }

    #[test]
    fn test_challenge_2() {
        assert_eq!(
            cryptopals::transform::hex_xor(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965"
            ),
            "746865206b696420646f6e277420706c6179"
        )
    }

    #[test]
    fn test_challenge_3() {
        let bytes = cryptopals::encode::hex_to_bytes(
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
        );

        let (_, output) = cryptopals::cracking::single_byte_cipher(&bytes);

        assert_eq!(
            output.iter().map(|byte| *byte as char).collect::<String>(),
            "Cooking MC's like a pound of bacon"
        )
    }

    #[test]
    fn test_challenge_4() {
        use std::fs;

        // 1. Split content by lines
        // 2. read hex into bytes
        // 3. for each line, find best single char cipher
        // 4. select best score overall
        let (_, bytes) = fs::read_to_string("4/data.txt")
            .unwrap()
            .split("\n")
            .map( |line| cryptopals::encode::hex_to_bytes(line) )
            .map( |bytes| cryptopals::cracking::single_byte_cipher(&bytes) )
            .max_by_key( |(s, _)| *s )
            .unwrap();

        assert_eq!(
            bytes.iter().map(|by| *by as char).collect::<String>(),
            "Now that the party is jumping\n"
        )
    }

    #[test]
    fn test_challenge_5() {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let result = cryptopals::encrypt::repeating_key_cipher(
            &input.as_bytes().to_vec(),
            "ICE"
        );

        assert_eq!(
            cryptopals::encode::bytes_to_hex(result),
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        )
    }

    #[test]
    fn test_challenge_6() {
        use std::fs;

        let content = fs::read_to_string("6/data.txt").unwrap();

        // let bytes = cryptopals::encode::b64_to_bytes(content.as_str());
    }
}
