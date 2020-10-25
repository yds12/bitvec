const BLOCK_SIZE: usize = 64;

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

  fn add_block(self: &mut Self) {
    self.data.push(0);
  }

  fn shift_left(self: &mut Self) {
    if self.data.len() * BLOCK_SIZE < self.length + 1 {
      self.add_block();
    }

    let data_len = self.data.len();

    for j in 0..data_len {
      let i: usize = data_len - j - 1;

      // shift left
      self.data[i] = self.data[i] << 1;

      // pull most significant element of previous block
      if i > 0 && self.data[i - 1] & (1 << (BLOCK_SIZE - 1)) > 0 {
        self.data[i] += 1;
      }
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
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_vec_has_len_zero() {
    let bitvec = BitVec::new();
    assert_eq!(bitvec.len(), 0);
  }

  #[test]
  fn len_after_push_one() {
    let mut bitvec = BitVec::new();
    bitvec.push(1);
    assert_eq!(bitvec.len(), 1);
  }

  #[test]
  #[should_panic]
  fn push_2() {
    let mut bitvec = BitVec::new();
    bitvec.push(2);
  }

  #[test]
  #[should_panic]
  fn push_45() {
    let mut bitvec = BitVec::new();
    bitvec.push(45);
  }

  #[test]
  #[should_panic]
  fn push_255() {
    let mut bitvec = BitVec::new();
    bitvec.push(255);
  }

  #[test]
  fn len_after_push_hundred() {
    let mut bitvec = BitVec::new();

    for i in 0..100 {
      bitvec.push(i % 2);
    }
    assert_eq!(bitvec.len(), 100);
  }

  #[test]
  fn push_one_block() {
    let mut bv = BitVec::new();
    bv.push(1);
    assert_eq!(bv.data[0], 1);

    let mut bv = BitVec::new();
    bv.push(1);
    bv.push(0);
    assert_eq!(bv.data[0], 2);

    let mut bv = BitVec::new();
    bv.push(1);
    bv.push(0);
    bv.push(1);
    assert_eq!(bv.data[0], 5);

    let mut bv = BitVec::new();
    bv.push(0);
    bv.push(1);
    bv.push(0);
    bv.push(1);
    assert_eq!(bv.data[0], 5);

    let mut bv = BitVec::new();
    bv.push(1);
    bv.push(0);
    bv.push(0);
    bv.push(1);
    bv.push(0);
    bv.push(1);
    bv.push(0);
    bv.push(0);
    assert_eq!(bv.data[0], 148);
  }

  #[test]
  fn push_two_blocks() {
    let mut bv = BitVec::new();
    bv.push(1);

    for _ in 0..64 {
      bv.push(0);
    }

    assert_eq!(bv.data[0], 0);
    assert_eq!(bv.data[1], 1);

    let mut bv = BitVec::new();
    bv.push(0);

    for _ in 0..64 {
      bv.push(1);
    }

    assert_eq!(bv.data[0], u64::MAX);
    assert_eq!(bv.data[1], 0);

    let mut bv = BitVec::new();
    bv.push(1);
    bv.push(1);

    for _ in 0..63 {
      bv.push(0);
    }

    assert_eq!(bv.data[0], u64::MAX / 2 + 1);
    assert_eq!(bv.data[1], 1);

    let mut bv = BitVec::new();

    for _ in 0..129 {
      bv.push(1);
    }

    assert_eq!(bv.data.len(), 3);
    assert_eq!(bv.data[2], 1);
    assert_eq!(bv.data[1], u64::MAX);
    assert_eq!(bv.data[0], u64::MAX);
  }
}

