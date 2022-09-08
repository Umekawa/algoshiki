use std::io;

fn main() {
  let mut input1 = String::new();
  io::stdin().read_line(&mut input1).ok();
  let num_s: String = input1.trim().to_string();
  let num: i32 = num_s.parse().unwrap();

  let mut left: i32 = 0;
  let mut right: i32 = 0;

  for n in 1..=num {
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).ok();
    let s: String = input2.trim().to_string();
    if (s == "right") {
      right+=1;
    } else if (s == "left") {
      left+=1;
    }
  }
  if (left > right) {
    println!("left");
  } else if (right > left) {
    println!("right");
  } else {
    println!("same");
  }
}