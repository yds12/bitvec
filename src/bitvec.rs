use std::mem;

// TODO: make this dynamic
macro_rules! BLOCK_FMT {
    () => ("{:064b}")
}

type BlType = u64;
const BL_SIZE: usize = mem::size_of::<BlType>();

/// This is the main struct of this crate: a vector of bits.
///
/// Data is represented via an underlying vector of unsigned integers.
/// Each unsigned integer in this structure is called a "block". The
/// first element of the bitvec is the highest bit in block 0, and so on.
/// As the indices grow, we move right inside a block, and when a block is
/// over we move to the next block.
///
/// Assuming a block size of 8, these are some examples of how some bitvecs
/// are represented in the underlying vector of uints:
/// bitvec:    1
/// uint vec: [10000000], len 1
/// bitvec:    101
/// uint vec: [10100000], len 3
/// bitvec:    10001000  1
/// uint vec: [10001000, 10000000], len 9
/// ...
#[derive(Debug)]
#[derive(Clone)]
pub struct BitVec {
  data: Vec<BlType>,
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
    let len = (self.length / 8) + ((self.length % 8 != 0) as usize);
    let bytevec: Vec<u8> = vec![0; len];
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

    let bl_index = index / BL_SIZE;
    let el_index = index % BL_SIZE;

    return ((self.data[bl_index] >> (BL_SIZE - 1 - el_index)) & 1) as u8;
  }

  fn add_block(self: &mut Self) {
    self.data.push(0);
  }

  fn remove_block(self: &mut Self) {
    self.data.pop();
  }

  pub fn push_many(self: &mut Self, value: u8, n: usize) {
  }

  pub fn len(self: Self) -> usize {
    self.length
  }

  /// Puts a bit at the end of the bitvec.
  pub fn push(self: &mut Self, value: u8) {
    if value > 1 {
      panic!("Only 0 and 1 can be pushed into BitVec.");
    }

    let bl_index = self.length / BL_SIZE;
    let el_index = self.length % BL_SIZE;
    let val = (1 as BlType) << (BL_SIZE - 1 - el_index);

    if bl_index >= self.data.len() {
      self.add_block();
    }

    if value == 1 {
      self.data[bl_index] = self.data[bl_index] | val;
    } else {
      // change the new bit even if it is 0
      self.data[bl_index] = self.data[bl_index] & (!val);
    }
    self.length += 1;
  }

  pub fn pop(self: &mut Self) -> Option<u8> {
    if self.length == 0 {
      return None;
    }

    let bl_index = (self.length - 1) / BL_SIZE;
    let el_index = (self.length - 1) % BL_SIZE;
    let val = ((self.data[bl_index] >> (BL_SIZE - 1 - el_index)) & 1) as u8;
    self.length -= 1;

    if self.length <= (self.data.len() - 1) * BL_SIZE {
      self.remove_block();
    }

    return Some(val);
  }

  pub fn push_byte(self: &mut Self, value: u8) {
    let bl_index = self.length / BL_SIZE;
    let el_index = self.length % BL_SIZE;
    let fits = self.data.len() * BL_SIZE - self.length;

    if fits < 8 { // needs new block to fit a byte
      self.add_block();
    }

    if fits < 8 && fits > 0 { // split bit in 2 parts
      // add part that fits in the last block
      let left = 8 - fits;
      let shift = (BL_SIZE - fits) - el_index;
      let part1 = ((value >> left) as BlType) << shift;
      self.data[bl_index] += part1;

      // add part that goes to the new block
      let shift = BL_SIZE - left;
      let part2 = (((u8::MAX >> fits) & value) << shift) as BlType;
      self.data[bl_index + 1] += part2;
    } else {
      let shift = (BL_SIZE - 8) - el_index;
      let add = (value as BlType) << shift;
      self.data[bl_index] += add;
    }

    self.length += 8;
  }

  pub fn to_string(self: &Self) -> String {
    if self.length == 0 {
      return String::from("");
    }

    let last_block_size = match self.length % BL_SIZE {
      0 => BL_SIZE,
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

