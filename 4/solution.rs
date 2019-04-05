use std::fs::{self};

// Properly decode hex into bytes
fn hex_to_bytes(input: &str) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    let mut buf: u8 = 0;

    // Take two characters (4-byte nibbles) at a time and push into vector
    for (i, byte) in input.chars().enumerate() {
        let int = byte.to_digit(16).unwrap() as u8;
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

// XOR each byte against the test key
fn cribdrag_byte(a: &Vec<u8>, b: u8) -> Vec<u8> {
    a.iter().map(|a| a ^ b).collect()
}


// Score strings higher for having chars earlier in the frequency dict
fn english_score(input: &Vec<u8>) -> usize {
    let mut score = 0;
    // This is the string "etaoin shrdlu" as bytes. Searching through
    // a proper string and type casting to bytes turns out to be
    // really inefficient
    let freq: Vec<u8> = vec![101, 116, 97, 111, 105, 110, 32, 115,
                             104, 114, 100, 108, 117];
    let upper = freq.len();

    for byte in input.iter() {
        match freq.binary_search(byte) {
            Ok(i) => score += upper - i,
            Err(_) => {}
        }
    }

    score
}

fn best_score(input: &Vec<Vec<u8>>) -> &Vec<u8> {
    let mut high_score = 0;
    let mut best: &Vec<u8> = &input[0];

    for el in input.iter() {
        let score = english_score(&el);
        if score > high_score {
            high_score = score;
            best = el;
        }
    }

    best
}

fn decrypt(input: &str) -> Vec<u8> {
    let bytes = hex_to_bytes(&input);
    let dict: Vec<u8> = (0x00..0xFF).collect();

    let mut results: Vec<Vec<u8>> = Vec::new();

    for key in dict.iter() {
        results.push(cribdrag_byte(&bytes, *key))
    }

    best_score(&results).to_vec()
}

fn main() {
    let file = fs::read_to_string("data.txt").unwrap();
    let lines: Vec<&str> = file.split("\n").collect();

    let mut decryptions: Vec<Vec<u8>> = Vec::new();
    for line in lines.iter() {
        decryptions.push(decrypt(line));
    }

    let winner = best_score(&decryptions);
    let score = english_score(&winner);
    let output: String = winner.iter().map(|byte| *byte as char).collect();
    println!("({}) {}", score, output);
}
