pub fn ascii(msg: String) {
  let mut block = vec![0u32];
  let mut b = 0;

  let mut word: u32 = 0;

  for (mut _index, ch) in msg.chars().enumerate() {
    _index += 1;
    word = (word<<8) + (ch as u32);

    if _index % 4 == 0 {
      println!("w: {:?}", word);
      block.push(word);
      word = 0;

      if _index % 64 == 0 {

      }

      
    }
  }

  // let a: u8 = 97;
  // let b: u8 = 98;
  // let c: u8 = 99;
  // let d: u8 = 100;

  // let mut block: [u32; 16] = [];
  
  

  
}