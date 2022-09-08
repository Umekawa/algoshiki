use std::io;

fn main() {
  let mut input1 = String::new();
  io::stdin().read_line(&mut input1).ok();
  let num_s: String = input1.trim().to_string();
  let num: i32 = num_s.parse().unwrap();

  let mut output = String::new();

  for n in 1..=num {
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).ok();
    let s: String = input2.trim().to_string();
    output.push(s.chars().nth(0).unwrap());
  }
  println!("{}", output);
}