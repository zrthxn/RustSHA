#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(dead_code)]
#[allow(unused_parens)]

mod core;

fn main() {
  core::hash("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890+/".to_string());
}