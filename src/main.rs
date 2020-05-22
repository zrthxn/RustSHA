#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(dead_code)]
#[allow(unused_parens)]

mod core;

fn main() {
  core::hash("gobbledygook".to_string());
  
  // bug: input module
  // inputs longer than 64 bytes arent encoded properly
  // core::hash("gobbledygookgobbledygookgobbledygookgobbledygookgobbledygookgobbledygookgobbledygookgobbledygookgobbledygook".to_string());
}