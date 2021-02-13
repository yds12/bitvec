const BLOCK_SIZE: usize = 64;

macro_rules! BLOCK_FMT {
    () => ("{:064b}")
}

#[derive(Debug)]
#[derive(Clone)]
pub struct BitVec {
  data: Vec<u64>,
  length: usize
}

impl BitVec {
  pub fn new() -> Self {
    BitVec {
      data: Vec::new(),
      length: 0
    }
  }

  pub fn from_str(strval: &str) -> Self {
    BitVec::from_bytes(strval.as_bytes())
  }

  pub fn from_bytes(bytes: &[u8]) -> Self {
    let mut bv = BitVec::new();

    for &byte in bytes {
      bv.push_byte(byte);
    }

    bv
  }

  pub fn to_vecu8(self: &Self) -> Vec<u8> {
//println!("");
//println!("data: {:?}", self.data);
//println!("bv: {}", self.to_string());
    let len = (self.length / 8) + ((self.length % 8 != 0) as usize);
    let mut bytevec: Vec<u8> = vec![0; len];

    for i in 0..len { // for each byte
      let mut mask: u8 = 0xff;
      let bl_idx = i / 8; // index of the data block
      let byte_in_block = i % (BLOCK_SIZE / 8);

      let shift = // how much to shift to make this the lowest byte
        if bl_idx < self.data.len() - 1 { // not the last block
          (7 - byte_in_block) * 8
        } else { // last block
          let bits_in_block = if self.length % BLOCK_SIZE == 0 {
              BLOCK_SIZE
            } else {
              self.length % BLOCK_SIZE
            };


          if i == len - 1 {
            let byte_size =
              if self.length % 8 == 0 { 8 } else { self.length % 8 };

            mask = (2_u32.pow(byte_size as u32) - 1) as u8;
            0
          } else {
            bits_in_block - ((byte_in_block + 1) * 8)
          }
        };
//println!("byte: {}", i);
//println!("block: {}", bl_idx);
//println!("shift: {}", shift);
//println!("mask: 0x{:x}", mask);

      bytevec[i] = (self.data[bl_idx] >> shift) as u8 & mask;
//println!("value: {}", bytevec[i]);
    }

    bytevec
  }

  pub fn get(self: &Self, index: usize) -> u8 {
    if self.length == 0 {
      panic!("Cannot index into empty BitVec.");
    }

    if index >= self.length {
      panic!("Index for BitVec out of bounds. Length: {}, index: {}",
        self.length, index);
    }

    let last_block_size = match self.length % BLOCK_SIZE {
      0 => BLOCK_SIZE,
      val => val
    };
    let block_index = (self.length - index - 1) / BLOCK_SIZE;
    let el_index =
      (BLOCK_SIZE - ((BLOCK_SIZE + index - (last_block_size - 1))
      % BLOCK_SIZE)) % BLOCK_SIZE;

    return ((self.data[block_index] >> el_index) % 2) as u8;
  }

  fn add_block(self: &mut Self) {
    self.data.push(0);
  }

  fn remove_block(self: &mut Self) {
    self.data.pop();
  }

  fn shift_left(self: &mut Self) {
    if self.data.len() * BLOCK_SIZE < self.length + 1 {
      self.add_block();
    }

    let data_len = self.data.len();

    for j in 0..data_len {
      let i: usize = data_len - j - 1;

      self.data[i] = self.data[i] << 1;

      // pull most significant element of previous block
      if i > 0 && self.data[i - 1] & (1 << (BLOCK_SIZE - 1)) > 0 {
        self.data[i] += 1;
      }
    }
  }

  fn shift_right(self: &mut Self) {
    let data_len = self.data.len();

    let mut carry = false;

    for j in 0..data_len {
      let i: usize = data_len - j - 1;

      if carry {
        self.data[i] += 1_u64.rotate_right(1);
      }
      carry = self.data[i] % 2 == 1;

      self.data[i] = self.data[i] >> 1;
    }
  }

  fn shift_byte_left(self: &mut Self) {
    if self.data.len() * BLOCK_SIZE < self.length + 8 {
      self.add_block();
    }

    let data_len = self.data.len();

    for j in 0..data_len {
      let i: usize = data_len - j - 1;

      // shift left
      self.data[i] = self.data[i] << 8;

      // pull most significant elements of previous block
      if i > 0 {
        self.data[i] += self.data[i - 1] >> (BLOCK_SIZE - 8);
      }
    }
  }

  fn shift_block_left(self: &mut Self) {
    self.data.insert(0, 0);
  }

  fn shift_n_left(self: &mut Self, n: usize) {
    for _ in 0..(n / BLOCK_SIZE) {
      self.shift_block_left();
    }

    let remains = n % BLOCK_SIZE;

    let last_block_size = match self.length % BLOCK_SIZE {
      0 => BLOCK_SIZE,
      val => val
    };

    if last_block_size + remains > BLOCK_SIZE {
      self.add_block();
    }

    let data_len = self.data.len();

    for j in 0..data_len {
      let i: usize = data_len - j - 1;

      self.data[i] = self.data[i] << remains;

      // pull most significant elements of previous block
      if i > 0 {
        self.data[i] += self.data[i - 1] >> (BLOCK_SIZE - remains);
      }
    }

    self.length += n;
  }

  pub fn set_block(self: &mut Self, index: usize, value: u8) {
    if value > 1 {
      panic!("BitVec only accepts values 0 and 1.");
    }

    self.data[index] = if value == 1 { u64::MAX } else { 0 };
  }

  pub fn append_many(self: &mut Self, value: u8, n: usize) {
    if value > 1 {
      panic!("BitVec only accepts values 0 and 1.");
    }

    self.shift_n_left(n);
    let blocks = n / BLOCK_SIZE;
    let remains = n % BLOCK_SIZE;

    for i in 0..blocks {
      self.set_block(i, value);
    }

    if value == 1 {
      self.data[blocks] += u64::MAX >> (BLOCK_SIZE - remains);
    }
  }

  pub fn len(self: Self) -> usize {
    self.length
  }

  pub fn push(self: &mut Self, value: u8) {
    if value > 1 {
      panic!("Only 0 and 1 can be pushed into BitVec.");
    }
    self.shift_left();
    self.data[0] += value as u64;
    self.length += 1;
  }

  pub fn pop(self: &mut Self) -> Option<u8> {
    if self.length == 0 {
      return None;
    }

    let val = if self.data[0] % 2 == 0 { 0 } else { 1 };
    self.shift_right();
    self.length -= 1;

    if self.length <= (self.data.len() - 1) * BLOCK_SIZE {
      self.remove_block();
    }

    return Some(val);
  }

  pub fn push_byte(self: &mut Self, value: u8) {
    self.shift_byte_left();
    self.data[0] += value as u64;
    self.length += 8;
  }

  pub fn to_string(self: &Self) -> String {
    if self.length == 0 {
      return String::from("");
    }

    let last_block_size = match self.length % BLOCK_SIZE {
      0 => BLOCK_SIZE,
      val => val
    };
    let mut s = format!("{:0width$b}", self.data[self.data.len() - 1],
      width = last_block_size);

    for i in 0..(self.data.len() - 1) {
      s = s + &(format!(BLOCK_FMT!(), self.data[self.data.len() - i - 2]));
    }
    s
  }
}

mod tests;

