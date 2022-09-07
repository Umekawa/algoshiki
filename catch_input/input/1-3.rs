use std::io;

fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).ok();
  let s: &str  = input.trim();
  println!("{}", format!("{}{}{}",s,s,s));
}
