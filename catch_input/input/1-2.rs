use std::io;

fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).ok();
  let num: i32 = input.trim().parse().ok().unwrap();
  println!("{}", (num%5).to_string());
}
