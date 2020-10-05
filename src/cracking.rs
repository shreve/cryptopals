use crate::Bytes;
use crate::{encrypt, transform};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn single_byte_cipher(input: &Bytes) -> (usize, u8, Bytes) {
    let dict: Bytes = (0x00..0xFF).collect();

    let mut high_score = 0;
    let mut key: u8 = 0;
    let mut best: Bytes = vec![0; input.len()];

    for chr in dict.iter() {
        let output = transform::char_xor(input, *chr);
        let score = english_score(&output);

        if score > high_score {
            high_score = score;
            best = output;
            key = *chr;
        }
    }

    (high_score, key, best)
}

pub fn repeating_key_cipher(data: &Bytes) -> (Bytes, Bytes) {
    let len = find_key_length(data, 1)[0];

    let mut map: Vec<Bytes> = vec![Vec::new(); len];

    let chunks = data.chunks(len);

    for chunk in chunks {
        for (i, byte) in chunk.iter().enumerate() {
            map[i].push(*byte);
        }
    }

    let cipher_key: Bytes = map
        .iter()
        .map(|cipher| single_byte_cipher(cipher))
        .map(|(_, key, _)| key)
        .collect();

    let plaintext = encrypt::repeating_key_cipher(data, &cipher_key);

    (cipher_key, plaintext)
}

pub fn find_ecb(data: &Vec<Bytes>) -> Bytes {
    use std::collections::HashSet;

    for line in data {
        let mut set = HashSet::new();
        let chunks: Vec<&[u8]> = line.chunks_exact(16).collect();
        for chunk in chunks.iter() {
            set.insert(chunk.to_vec());
        }

        if set.len() < chunks.len() {
            return line.to_vec();
        }
    }

    Bytes::new()
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

#[derive(Copy, Clone, Eq, PartialEq)]
struct LenDist {
    len: usize,
    dist: usize,
}

impl Ord for LenDist {
    fn cmp(&self, other: &LenDist) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for LenDist {
    fn partial_cmp(&self, other: &LenDist) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_key_length(data: &Bytes, count: usize) -> Vec<usize> {
    let mut heap = BinaryHeap::new();

    for len in 2..=40 {
        let mut iter = data.chunks_exact(len);
        let mut dists: Vec<f64> = Vec::new();

        loop {
            match (iter.next(), iter.next()) {
                (Some(first), Some(second)) => {
                    dists.push(
                        hamming_distance(&first.to_vec(), &second.to_vec())
                            as f64
                            / len as f64,
                    );
                }
                (_, _) => break,
            }
        }

        let average: f64 = dists.iter().sum::<f64>() / dists.len() as f64;
        let dist = (average * 10000.0) as usize;

        heap.push(LenDist { len, dist });
    }

    let mut result = Vec::new();
    for _ in 1..=count {
        if let Some(pair) = heap.pop() {
            result.push(pair.len);
        }
    }

    result
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
