use std::u8;
use std::env;

fn hex_to_64(input: String) -> String {

    // Collect the base64 alphabet as bytes
    let b64: Vec<u8> =
        (0x41..0x5B)
        .chain(0x61..0x7B)
        .chain(0x30..0x3A)
        .chain(0x2B..0x2C)
        .chain(0x2F..0x30)
        .collect();

    // Instantiate the output and bit buffers
    let mut output = "".to_string();
    let mut buf: u32 = 0;

    // Iterate through each character in the input
    for (i, char) in input.char_indices() {

        // For each byte, pull out the last 4 bits (2^4 = 16)
        match char.to_digit(16) {
            Some(byte) => {
                buf <<= 4;
                buf |= byte;
            },
            None => {}
        }

        // Every 3 chars we have 12 bits in the buffer, so pull out 2 b64 bytes
        if (i + 1) % 3 == 0 {

            // 4032 is top 6 bits on, 63 is bottom 6 bits on
            let m: u8 = ((buf & 4032) >> 6) as u8;
            let n: u8 = (buf & 63) as u8;
            output.push(b64[m as usize] as char);
            output.push(b64[n as usize] as char);
            buf = 0;
        }
    }

    output
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(input) => println!("{}", hex_to_64(input.to_string())),
        None => println!("Program needs one input"),
    }
}
