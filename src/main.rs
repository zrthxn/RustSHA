#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(dead_code)]
#[allow(unused_parens)]

mod func;
mod input;

fn main() {
  // let a: u8 = 97;
  // let b: u8 = 98;
  // let c: u8 = 99;
  
  // let mut word: u32 = 0;
  // word = (word*(2<<8)) + (a as u32);
  // word = (word*(2<<8)) + (b as u32);
  // word = (word*(2<<8)) + (c as u32);

  // println!("Main: {}", func::rotr(word as u32, 3));

  input::ascii("abcdefgh".to_string())
}