use std::{char, env, process};

fn hex_to_bytes(input: String) -> Vec<u8> {
    input.chars().map(|char| char.to_digit(16).unwrap() as u8).collect()
}

fn bytes_to_hex(input: Vec<u8>) -> String {
    input.iter().map(|byte| char::from_digit(*byte as u32, 16).unwrap()).collect()
}

// Zip the vectors together and xor each byte pair
fn byte_xor(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    a.iter().zip(b).map(|(a,b)| (a ^ b)).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match (args.get(1), args.get(2)) {
        (Some(a), Some(b)) => {

            // Convert inputs to bytes, xor them, then convert back to hex
            let output = bytes_to_hex(
                byte_xor(
                    hex_to_bytes(a.to_string()),
                    hex_to_bytes(b.to_string())
                )
            );
            println!("{}", output);
        },
        (_, _) => {
            println!("Program needs two inputs");
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correctness() {
        let output = bytes_to_hex(
        );
        assert_eq!(output,)
    }
}
