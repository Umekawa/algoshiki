use std::io;

fn main() {
  let mut input = String::new();
  let mut input2 = String::new();
  io::stdin().read_line(&mut input).ok();
  io::stdin().read_line(&mut input2).ok();
  let s1: String = input.trim().to_string();
  let s2: String = input2.trim().to_string();
  if (s1 == s2) {
    println!("Yes");
  } else {
    println!("No");
  }
}
