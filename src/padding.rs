use crate::Bytes;

pub fn pkcs7(bytes: &Bytes, len: u8) -> Bytes {
  let mut out = bytes.clone();
  let to_add: u8 = len - (bytes.len() as u8 % len);
  println!("{:?}", to_add);
  println!("{:?}", out);
  let mut add_vec = vec![to_add; to_add as usize];
  out.append(&mut add_vec);
  println!("{:?}", out);
  out
}
