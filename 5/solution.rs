use std::{char, env, process};

fn repeating_key_xor(key: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let keylen = key.len();
    let msglen = plaintext.len();
    let mut output = Vec::new();

    for i in 0..msglen {
        let j = i % keylen;

        output.push(key[j] ^ plaintext[i]);
    }

    output
}

fn bytes_to_hex(bytes: &Vec<u8>) -> String {
    let mut output = Vec::new();

    for byte in bytes.iter() {
        output.push(char::from_digit(((*byte & 240) >> 4) as u32, 16).unwrap());
        output.push(char::from_digit((*byte & 15) as u32, 16).unwrap());
    }

    output.iter().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match (args.get(1), args.get(2)) {
        (Some(key), Some(plaintext)) => {
            let output = repeating_key_xor(key.as_bytes(), plaintext.as_bytes());
            println!("{}", bytes_to_hex(&output));
        },
        (_, _) => {
            println!("Program needs two inputs");
            process::exit(1);
        }
    }
}
