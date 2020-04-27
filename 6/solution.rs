use std::fs::{self};

// Based on the hamming distance example algorithm on Wikipedia
// https://en.wikipedia.org/wiki/Hamming_distance#Algorithm_example
fn hamming_distance(left: &[u8], right: &[u8]) -> usize {
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

fn b64_to_bytes(input: String) -> Vec<u8> {
    let mut buf: u32 = 0;
    let mut output = Vec::new();

    for (i, chr) in input.chars().enumerate() {
        buf <<= 6;
        buf |= chr as u32;

        if (i+1) % 4 == 0 {
            let mut unload = Vec::new();
            for _ in 0..3 {
                let m = buf & 255;
                unload.push(m as u8);
                buf >>= 8;
            }
            unload.reverse();
            for byte in unload.iter() {
                output.push(*byte);
            }
        }
    }

    output
}

fn key_length(bytes: &Vec<u8>) -> usize {
    let mut min_dist = 99999.0;
    let mut length = 0;

    for len in 2..=40 {
        let mut iter = bytes.chunks(len);
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        let third = iter.next().unwrap();
        let fourth = iter.next().unwrap();
        let dist = (hamming_distance(first, second) +
            hamming_distance(third, fourth)) as f64 / (2 * len) as f64;

        if dist < min_dist {
            min_dist = dist;
            length = len;
        }
    }

    length
}

fn find_key(bytes: &Vec<u8>, size: usize) -> Vec<u8> {
    let iter = bytes.chunks(size);
    let mut map: Vec<Vec<u8>> = vec![Vec::new(); size];

    for chunk in iter {
        for (i, byte) in chunk.iter().enumerate() {
            map[i].push(*byte);
        }
    }

    map[0].to_vec()
}

fn main() {
    let file = fs::read_to_string("data.txt").unwrap();
    let bytes = b64_to_bytes(file);
    let keysize = key_length(&bytes);
    let key = find_key(&bytes, keysize);
    println!("Key length: {}", keysize);
}
