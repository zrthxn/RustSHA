pub fn ascii(msg: String) -> Vec<Vec<u32>> {
  let mut list = Vec::new();
  let mut block = Vec::new();
  let mut word: u32 = 0;
  let mut byte = 0;

  for ch in msg.chars() {
    word = (word<<8) + (ch as u32);
    byte += 1;

    if byte < msg.len() {
      if byte % 4 == 0 {
        block.push(word);
        word = 0u32;

        if block.len() == 16 {
          list.push(block);
          block = Vec::new();
        }
      }
    }
  }

  if byte % 4 != 0 {
    // pad current word if incomplete
    word = word << 1;
    word += 1u32;

    word = word << ((8 * (4 - byte)) - 1);

    if block.len() == (16 - 1) {
      // put length in last byte if last of block
      word += (((byte - 1) % 64) * 8) as u32;
    } 
  }

  block.push(word);

  if block.len() < 16 {
    let mut lword: u32 = 0u32;
    if byte % 4 == 0 {
      lword = 1u32<<(32-1);
    }

    if block.len() < 15 {
      block.push(lword);
      for _ in block.len()..15 {
        block.push(0u32);
      }

      block.push(((byte % 64) * 8) as u32);
    } else {
      lword += ((byte % 64) * 8) as u32;
      block.push(lword);
    }
  }
  
  list.push(block);
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