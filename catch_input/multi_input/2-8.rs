use std::io;

fn main() {
  let mut input1 = String::new();
  let mut input2 = String::new();
  let mut input3 = String::new();
  io::stdin().read_line(&mut input1).ok();
  io::stdin().read_line(&mut input2).ok();
  io::stdin().read_line(&mut input3).ok();
  let s1: String = input1.trim().to_string();
  let s2: String = input2.trim().to_string();
  let s3: String = input3.trim().to_string();
  println!("{}{}{}", s3, s2, s1);
}