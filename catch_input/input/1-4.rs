use std::io;

fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).ok();
  let v: Vec<char> = input.chars().collect();
  println!("{}", v[2]);
}
