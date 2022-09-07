use std::io;

fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).ok();
  let s: String = input.trim().to_string();
  let mut vec: Vec<&str> = s.split(' ').collect();
  let b: i32 = vec.pop().unwrap().parse().unwrap();
  let a: i32 = vec.pop().unwrap().parse().unwrap();
  if b > a  {
    println!("{}",b.to_string());
  } else {
    println!("{}",a.to_string());
  }  
}
