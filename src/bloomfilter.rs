use std::{
  borrow::Borrow,
  hash::{DefaultHasher, Hash, Hasher},
  marker::PhantomData,
};

use num_traits::Euclid;

pub struct BloomFilter<T> {
  bits: Vec<u64>,
  hashes: usize,
  _marker: PhantomData<T>,
}

const BITS: usize = std::mem::size_of::<u64>();

impl<T: Hash> BloomFilter<T> {
  pub fn new(bits: usize, hashes: usize) -> Self {
    Self {
      bits: vec![0; bits.div_ceil(BITS)],
      hashes,
      _marker: PhantomData,
    }
  }

  fn hash(item: &T, offset: usize) -> usize {
    let mut s = DefaultHasher::new();
    offset.hash(&mut s);
    (*item).hash(&mut s);
    s.finish() as usize
  }

  pub fn num_bits(&self) -> usize {
    self.bits.len() * BITS
  }

  pub fn num_set_bits(&self) -> usize {
    self
      .bits
      .iter()
      .map(|word| word.count_ones() as usize)
      .sum()
  }

  pub fn insert(&mut self, item: impl Borrow<T>) {
    let num_bits = self.num_bits();
    (0..self.hashes)
      .map(|x| Self::hash(item.borrow(), x))
      .map(|x| x % num_bits)
      .for_each(|idx| {
        eprintln!("{}", idx);
        let (word, bit) = idx.div_rem_euclid(&BITS);
        self.bits[word] |= 0x1 << bit;
      })
  }

  pub fn has(&self, item: impl Borrow<T>) -> bool {
    let num_bits = self.num_bits();
    (0..self.hashes)
      .map(|x| Self::hash(item.borrow(), x))
      .map(|x| x % num_bits)
      .all(|idx| {
        eprintln!("{}", idx);
        let (word, bit) = idx.div_rem_euclid(&BITS);
        self.bits[word] & !(0x1 << bit) != 0
      })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_construction() {
    let mut bf: BloomFilter<u32> = BloomFilter::new(1024, 3);
    bf.insert(404);
    bf.insert(1013);
    bf.insert(234823);

    assert!(bf.has(404));
    assert!(bf.has(1013));
    assert!(bf.has(234823));
  }
}
