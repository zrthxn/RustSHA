// 0. init constants (k, 64) with cube roots of first 64 primes
// 1. break msg into 512bit blocks and add padding
//// 2. create msg schedule: split into 16 32bit words
//// 3. complete schedule, apply transforms
//// 4. init state registers with sqr roots of primes, take fractional parts
//// 5. create temporary words and apply state table transforms
//// 6. add transformed state table to init
// 7. if no further blocks, digest by concat ascii repr of state registers

mod func;
mod input;

pub fn hash(message: String) {
  let blocks = input::ascii(message);
  println!("List ({:?}): {:?}", blocks.len(), blocks);

  for block in blocks {
    
  }
}

fn schedule(block: Vec<u32>) -> Vec<u32> {
  unimplemented!();
}