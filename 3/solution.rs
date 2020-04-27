#[allow(dead_code)]

use std::{env, process};

// Properly decode hex into bytes
fn hex_to_bytes(input: &String) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    let mut buf: u8 = 0;

    // Take two characters (4-byte nibbles) at a time and push into vector
    for (i, char) in input.char_indices() {
        let int = char.to_digit(16).unwrap() as u8;
        if i % 2 == 0 {
            buf |= int << 4;
        } else {
            buf |= int;
            output.push(buf);
            buf = 0;
        }
    }

    output
}

// Zip the vectors together and xor each byte pair
fn byte_xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.iter().zip(b).map(|(a,b)| (a ^ b)).collect()
}

const FREQ: &str = "etaoin shrdlucmfwypvbgkjqxz";

// Score strings higher for having characters earlier in the frequency dict
fn english_score(input: &String) -> usize {
    let mut score = 0;

    for char in input.chars() {
        match FREQ.chars().position(|c| c == char) {
            Some(i) => score += 30 - i,
            None => {}
        }
    }

    score
}

fn decrypt(input: String) -> String {
    let bytes = hex_to_bytes(&input);
    println!("{:?}", bytes);
    let dict: Vec<u8> = (0x00..0xFF).collect();

    let mut high_score = 0;
    let mut best: String = "".to_string();

    for key in dict.iter() {

        // Generate an input-length string of the key repeated
        let key_string = vec![*key; input.len()];

        // Perform the xor and convert the output to a character string
        let output: String = byte_xor(&bytes, &key_string)
            .iter().map(|byte| *byte as char).collect();

        // Save the best-scoring output
        let score = english_score(&output);
        if score > high_score {
            high_score = score;
            best = output;
        }
    }

    best
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(input) => {
            println!("{}", decrypt(input.to_string()));
        },
        _ => {
            println!("Program needs one input");
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correctness() {
        let output = decrypt("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string());
        assert_eq!(output, "Cooking MC's like a pound of bacon")
    }
}
