use std::io;

fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).ok();
  let s: String = input.trim().to_string();
  let mut vec: Vec<&str> = s.split(' ').collect();
  let v2: Vec<i32> = vec.iter().map(|x| x.parse().unwrap()).collect()
  println!("{}", v2.iter().max().unwrap());
}
