#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_vec_has_len_zero() {
    let bitvec = BitVec::new();
    assert_eq!(bitvec.len(), 0);
  }
}

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
    self.data.insert(0, 0);
  }

  pub fn len(self: Self) -> usize {
    self.length
  }

  pub fn push(self: &mut Self, value: u8) {
    if self.data.len() * 64 < self.length + 1 {
      self.add_block();
    }
    self.length += 1;
  }
}
