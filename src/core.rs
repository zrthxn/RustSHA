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

// Constants
const K: [u32; 64] = [
  0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
  0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
  0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
  0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
  0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
  0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
  0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
  0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2 
];

// Initial State Table Values
const H: [u32; 8] = [
  0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19
];

pub fn hash(message: String) {
  let blocks = input::ascii(message);
  println!("List ({:?}): {:?}", blocks.len(), blocks);

  for mut block in blocks {
    block = schedule(block);
    
  }
}

fn schedule(mut block: Vec<u32>) -> Vec<u32> {
  for mut ti in 16..64 {
    block.push (
      (
        func::sig1( block[ti - 2] ) as u64 +
        block[ti - 7] as u64 +
        func::sig0( block[ti - 15] ) as u64 + 
        block[ti - 16] as u64
      ) as u32
    );
  }
  return block
}

// @deprecated
// Initialize const values
fn init() {
  let mut pi: f32 = 1.0;

  let mut ki = 0;
  let mut hi = 0;

  loop {
    pi += 1.0;
    
    if _is_prime(pi as u16) {
      if hi < H.len() {
        H[hi] = ((pi.sqrt().fract()) * (10.0 as f32).powi(8).trunc()) as u32;
        hi += 1;
        continue;
      }
      if ki < K.len() {
        K[ki] = ((pi.cbrt().fract()) * (10.0 as f32).powi(8).trunc()) as u32;
        ki += 1;
        continue;
      }
    }
    break;
  }
}

fn _is_prime(x: u16) -> bool {
  let mut flag: bool = true;
  
  if x == 0 || x == 1 {
    return false;
  }
  if x == 2 || x == 3 || x == 5 {
    return true;
  }

  if x % 2 == 0 {
    for i in 2..(x/2) {
      if x % i == 0 {
        flag = false;
      }
    }
  } else {
    for i in 2..((x - 1)/2) {
      if x % i == 0 {
        flag = false;
      }
    }
  }

  return flag
}