use std::io::{self, Read, Write};
use std::fs;

const NEEDLE: &'static str = "/**===============**/";

fn main() {
  let mut buffer = String::new();
  let mut stdin = io::stdin(); // We get `Stdin` here.
  stdin.read_to_string(&mut buffer).unwrap();

  let splited = buffer.split(NEEDLE);
  let str_vec = splited.into_iter().collect::<Vec<&str>>();
  for i in 0..str_vec.len() {
    let v = str_vec[i].trim();
    if v.len() != 0 {
      let mut file = fs::File::create(format!("loader/input{}.txt", i+1)).unwrap();
      file.write_all(v.as_bytes()).unwrap();
    }
  }
}
