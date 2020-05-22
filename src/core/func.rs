pub fn mux(c: u32, y: u32, z: u32) -> u32 {
  // 2:1 Multiplexer
  (!(
    (!(y&(c)))
      &
    (!(z&(!c)))
  )) as u32
}

pub fn maj(a: u32, b: u32, c: u32) -> u32 {
  // Majority function
  (
    (a&b) | (b&c) | (c&a)
  ) as u32
}

pub fn rotr(x: u32, r: u8) -> u32 {
  (x >> r) | (x << (32-r)) as u32
}

pub fn sig0(x: u32) -> u32 {
  // sigma 0 function
  (
    rotr(x, 7) ^
    rotr(x, 18) ^
    x>>3
  ) as u32
}

pub fn sig1(x: u32) -> u32 {
  // sigma 1 function
  (
    rotr(x, 17) ^
    rotr(x, 19) ^
    x>>10
  ) as u32
}

pub fn usig0(x: u32) -> u32 {
  // Upper SIGMA 0 function
  (
    rotr(x, 2) ^
    rotr(x, 13) ^
    rotr(x, 22)
  ) as u32
}

pub fn usig1(x: u32) -> u32 {
  // Upper SIGMA 1 function
  (
    rotr(x, 6) ^
    rotr(x, 11) ^
    rotr(x, 25)
  ) as u32
}