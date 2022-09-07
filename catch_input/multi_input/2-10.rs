use std::io;

fn main() {
  let mut input1 = String::new();
  io::stdin().read_line(&mut input1).ok();
  let s1: String = input1.trim().to_string();
  let cs: Vec<char> = s1.chars().collect();

  let mut input2 = String::new();
  io::stdin().read_line(&mut input2).ok();
  let s: String = input2.trim().to_string();
  let mut vec: Vec<&str> = s.split(' ').collect();
  let mut v2: Vec<i32> = vec.iter().map(|x| x.parse().unwrap()).collect();
  let m: usize = v2.pop().unwrap() as usize;
  let n: usize = v2.pop().unwrap() as usize;

  let mut answer = String::new();

  for i in 0..s1.len() {
    if (i+1) ==m {
      answer.push(cs[n-1]);
    } else if (i+1)==n {
      answer.push(cs[m-1]);
    } else {
      answer.push(cs[i]);
    }
  }
    println!("{}", answer);
}