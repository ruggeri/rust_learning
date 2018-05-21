use std::io;

fn main() {
  println!("Hello! What is your name?");

  let mut buf = String::new();
  io::stdin().read_line(&mut buf).expect("Failed to read name!");

  println!("Your name is {}!", buf.trim());
}
