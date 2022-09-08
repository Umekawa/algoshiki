use std::io;

fn main() {
  let mut input1 = String::new();
  io::stdin().read_line(&mut input1).ok();

  let mut input2 = String::new();
  io::stdin().read_line(&mut input2).ok();
  let s: String = input2.trim().to_string();
  let mut vec: Vec<&str> = s.split(' ').collect();
  let mut v2: Vec<i32> = vec.iter().map(|x| x.parse().unwrap()).collect();
  let sum: i32 = v2.iter().sum();
  println!("{}", sum);
}