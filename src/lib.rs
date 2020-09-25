mod cracking;
mod decrypt;
mod encode;
mod encrypt;
mod transform;

type Bytes = Vec<u8>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_1() {
        assert_eq!(
      encode::hex_to_b64(
        "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
      ),
      "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    )
    }

    #[test]
    fn test_challenge_2() {
        assert_eq!(
            transform::hex_xor(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965"
            ),
            "746865206b696420646f6e277420706c6179"
        )
    }

    #[test]
    fn test_challenge_3() {
        let bytes = encode::hex_to_bytes(
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
        );

        let (_, _, output) = cracking::single_byte_cipher(&bytes);

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
        let (_, _, bytes) = fs::read_to_string("4/data.txt")
            .unwrap()
            .split("\n")
            .map(|line| encode::hex_to_bytes(line))
            .map(|bytes| cracking::single_byte_cipher(&bytes))
            .max_by_key(|(s, _, _)| *s)
            .unwrap();

        assert_eq!(
            bytes.iter().map(|by| *by as char).collect::<String>(),
            "Now that the party is jumping\n"
        )
    }

    #[test]
    fn test_challenge_5() {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let result = encrypt::repeating_key_cipher(
            &input.as_bytes().to_vec(),
            &"ICE".as_bytes().to_vec(),
        );

        assert_eq!(
      encode::bytes_to_hex(result),
      "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
    )
    }

    #[test]
    fn test_challenge_6() {
        use std::fs;

        let content = fs::read_to_string("6/data.txt").unwrap();
        let bytes = encode::b64_to_bytes(&content);
        let (key, _plaintext) = cracking::repeating_key_cipher(&bytes);

        assert_eq!(key, "Terminator X: Bring the noise".as_bytes());
    }

    #[test]
    fn test_challenge_7() {
        use std::fs;

        let content = fs::read_to_string("7/data.txt").unwrap();
        let bytes = encode::b64_to_bytes(&content);
        let plaintext =
            decrypt::aes_ecb(&bytes, &"YELLOW SUBMARINE".as_bytes().to_vec());

        assert_eq!(
            plaintext[0..33],
            *"I'm back and I'm ringin' the bell".as_bytes()
        );
    }

    #[test]
    fn test_challenge_8() {
        use std::fs;

        let content = fs::read_to_string("8/data.txt").unwrap();
        let entries = content
            .split("\n")
            .map(|s| encode::hex_to_bytes(s))
            .collect();
        let ecb_encrypted = cracking::find_ecb(&entries);
        assert_eq!(
            ecb_encrypted,
            vec![
                216, 128, 97, 151, 64, 168, 161, 155, 120, 64, 168, 163, 28,
                129, 10, 61, 8, 100, 154, 247, 13, 192, 111, 79, 213, 210, 214,
                156, 116, 76, 210, 131, 226, 221, 5, 47, 107, 100, 29, 191,
                157, 17, 176, 52, 133, 66, 187, 87, 8, 100, 154, 247, 13, 192,
                111, 79, 213, 210, 214, 156, 116, 76, 210, 131, 148, 117, 201,
                223, 219, 193, 212, 101, 151, 148, 157, 156, 126, 130, 191, 90,
                8, 100, 154, 247, 13, 192, 111, 79, 213, 210, 214, 156, 116,
                76, 210, 131, 151, 169, 62, 171, 141, 106, 236, 213, 102, 72,
                145, 84, 120, 154, 107, 3, 8, 100, 154, 247, 13, 192, 111, 79,
                213, 210, 214, 156, 116, 76, 210, 131, 212, 3, 24, 12, 152,
                200, 246, 219, 31, 42, 63, 156, 64, 64, 222, 176, 171, 81, 178,
                153, 51, 242, 193, 35, 197, 131, 134, 176, 111, 186, 24, 106
            ]
        )
    }
}
