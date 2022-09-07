use std::io;

fn main() {
  let mut input1 = String::new();
  let mut input2 = String::new();
  io::stdin().read_line(&mut input1).ok();
  io::stdin().read_line(&mut input2).ok();
  let s1: &str = input1.trim();
  let s2: &str = input2.trim();
  let n: i32 = s2.parse().unwrap();
  println!("{}",s1.as_bytes()[(n-1) as usize] as char);
}