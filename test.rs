static B64: &'static [u8] = &(0x41..0x5B).collect::<Vec<u8>>();

fn main() {
    println!("{:?}", B64);
}
