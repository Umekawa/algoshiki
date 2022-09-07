use std::io;

fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).ok();
  let s: String = input.trim().to_string();
  let mut vec: Vec<&str> = s.split(' ').collect();
  let a: i32 = vec.pop().unwrap().parse().unwrap();
  let b: i32 = vec.pop().unwrap().parse().unwrap();
  println!("{}",(a+b).to_string());
}
