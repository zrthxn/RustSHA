const BLOCK_BYTES: usize = 64;
const BLOCK_SIZE: usize = 16;
const BLOCK_TEXT: usize = 14;

pub fn ascii(string: String) -> Vec<Vec<u32>> {
  let mut message = Vec::new();
  for ch in string.chars() {
    message.push(ch as u8);
  }

  message.push( 1<<7 as u8 );

  let pad: usize = BLOCK_BYTES - ((message.len() + 8) % BLOCK_BYTES);

  for _ in 0..pad {
    message.push(0u8);
  }
  
  let mut len_bytes: [u8; 8] =  [ 0,0,0,0,0,0,0,0 ];
  for x in 0..8 {
    len_bytes[ 7 - x ] = (
      (string.len() * 8) / 
      (1 << (8 * x))
    ) as u8;
  }
  for y in 0..8 {
    message.push( len_bytes[y] )
  }

  let mut list = Vec::new();
  let mut block = Vec::new();
  let mut word: u32 = 0;

  for (mut count, byte) in message.iter().enumerate() {
    word = (word << 8) + (*byte as u32);
    count += 1;
    
    if count % 4 == 0 {
      block.push(word);
      word = 0u32;

      if block.len() == BLOCK_SIZE {
        list.push(block);
        block = Vec::new();
      }
    }
  }

  return list;
}

// fn pad_word(block: &Vec<u32>, word: &u32, i: u8) {
//   // pad current word if incomplete
//   if i % 4 != 0 {
//     word = word << (8 * (4 - (i % 4)));
//   }
//   if block.len() == (64 - 1) {
//     // put length in last byte
//     word += ((i % 16) * 8) as u32;
//   } 

//   block.push(word);
// }

// fn pad_block(word: &u32, pad: u8) {
//   // pad current word if incomplete
//   if byte % 4 != 0 {
//     word = word << (8 * (4 - (byte % 4)));
//   }
//   if block.len() == (64 - 1) {
//     // put length in last byte
//     word += ((byte % 16) * 8) as u32;
//   } 

//   block.push(word);

//   if block.len() < 64 {
//     // pad rest of block
//     for _ in block.len()..63 {
//       block.push(0u32);
//     }
//     block.push(((byte % 16) * 8) as u32);
//   }
// }