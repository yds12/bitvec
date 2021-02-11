#[cfg(test)]
mod tests {
  use crate::bitvec::*;

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

  #[test]
  fn get() {
    let mut bv = BitVec::new();
    bv.push(1);
    assert_eq!(bv.get(0), 1);

    let mut bv = BitVec::new();
    bv.push(0);
    assert_eq!(bv.get(0), 0);

    let mut bv = BitVec::new();
    bv.push(0);
    bv.push(1);
    bv.push(1);
    assert_eq!(bv.get(0), 0);
    assert_eq!(bv.get(1), 1);
    assert_eq!(bv.get(2), 1);

    let mut bv = BitVec::new();

    for _ in 0..63 {
      bv.push(0);
    }
    bv.push(1);

    for i in 0..63 {
      assert_eq!(bv.get(i), 0);
    }
    assert_eq!(bv.get(63), 1);

    let mut bv = BitVec::new();

    bv.push(1);
    for _ in 0..63 {
      bv.push(0);
    }
    bv.push(1);

    for i in 1..64 {
      assert_eq!(bv.get(i), 0);
    }
    assert_eq!(bv.get(0), 1);
    assert_eq!(bv.get(64), 1);

    let mut bv = BitVec::new();

    for _ in 0..128 {
      bv.push(0);
    }
    bv.push(1);
    bv.push(0);
    bv.push(1);
    bv.push(1);
    bv.push(0);

    assert_eq!(bv.get(128), 1);
    assert_eq!(bv.get(129), 0);
    assert_eq!(bv.get(130), 1);
    assert_eq!(bv.get(131), 1);
    assert_eq!(bv.get(132), 0);
  }

  #[test]
  fn to_string() {
    let mut bv = BitVec::new();
    bv.push(1);
    bv.push(0);
    bv.push(1);
    bv.push(1);
    bv.push(0);
    assert_eq!(bv.to_string(), "10110");

    let mut bv = BitVec::new();

    for _ in 0..128 {
      bv.push(0);
    }
    bv.push(1);
    bv.push(0);
    bv.push(1);
    bv.push(1);
    bv.push(0);

    assert_eq!(bv.to_string(),
      "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010110");

    let mut bv = BitVec::new();

    for _ in 0..10 {
      bv.push(0);
      bv.push(1);
      bv.push(0);
      bv.push(1);
      bv.push(1);
      bv.push(0);
      bv.push(1);
    }

    assert_eq!(bv.to_string(),
      "0101101010110101011010101101010110101011010101101010110101011010101101");
  }

  #[test]
  fn push_byte() {
    let mut bv = BitVec::new();
    bv.push_byte(1);
    bv.push_byte(1);
    bv.push_byte(1);
    bv.push_byte(1);

    for i in 0..32 {
      let val = if i % 8 == 7 { 1 } else { 0 };
      assert_eq!(bv.get(i), val);
    }

    let mut bv = BitVec::new();
    bv.push_byte(24);
    bv.push_byte(24);
    bv.push_byte(24);
    bv.push_byte(24);

    for i in 0..32 {
      let val = if i % 8 == 3 || i % 8 == 4 { 1 } else { 0 };
      assert_eq!(bv.get(i), val);
    }

    let mut bv = BitVec::new();
    bv.push_byte(128);
    bv.push_byte(16);
    bv.push_byte(2);
    bv.push_byte(64);
    let ones = vec![0, 11, 22, 25];

    for i in 0..32 {
      let val = if ones.contains(&i) { 1 } else { 0 };
      assert_eq!(bv.get(i), val);
    }

    let mut bv = BitVec::new();
    bv.push_byte(128);

    for _ in 0..8 {
      bv.push_byte(0);
    }

    assert_eq!(bv.get(0), 1);

    for i in 1..72 {
      assert_eq!(bv.get(i), 0);
    }

    let mut bv = BitVec::new();
    bv.push(1);
    for _ in 0..8 {
      bv.push_byte(0);
    }

    assert_eq!(bv.get(0), 1);

    for i in 1..65 {
      assert_eq!(bv.get(i), 0);
    }
  }

  #[test]
  fn pop_empty() {
    let mut bv = BitVec::new();
    assert_eq!(bv.pop(), None);
  }

  #[test]
  fn pop() {
    let mut bv = BitVec::new();
    bv.push(1);
    bv.push(0);
    bv.push(1);
    bv.push(1);
    bv.push(0);
    bv.push(0);
    assert_eq!(bv.pop().unwrap(), 0);
    assert_eq!(bv.pop().unwrap(), 0);
    assert_eq!(bv.pop().unwrap(), 1);
    assert_eq!(bv.pop().unwrap(), 1);
    assert_eq!(bv.pop().unwrap(), 0);
    assert_eq!(bv.pop().unwrap(), 1);
    assert_eq!(bv.pop(), None);
  }

  #[test]
  fn append_many() {
    let mut bv = BitVec::new();
    bv.push(1);
    bv.append_many(0, 10);
    bv.append_many(1, 10);

    assert_eq!(bv.get(0), 1);

    for i in 0..10 {
      assert_eq!(bv.get(i + 1), 0);
    }

    for i in 0..10 {
      assert_eq!(bv.get(i + 11), 1);
    }

    let mut bv = BitVec::new();
    bv.push(1);
    bv.append_many(0, 100);
    bv.append_many(1, 100);

    assert_eq!(bv.get(0), 1);

    for i in 0..100 {
      assert_eq!(bv.get(i + 1), 0);
    }

    for i in 0..100 {
      assert_eq!(bv.get(i + 101), 1);
    }
  }

  #[test]
  fn from_bytes() {
    let bytes: [u8; 3] = [0, 1, 2];
    let bv = BitVec::from_bytes(&bytes[..]);
    assert_eq!(bv.to_string(), "000000000000000100000010");
  }

  #[test]
  fn from_str() {
    let some_str = "abc";
    let bv = BitVec::from_str(some_str);
    assert_eq!(bv.to_string(), "011000010110001001100011");
  }
}

