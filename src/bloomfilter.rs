use std::{
  borrow::Borrow,
  hash::{DefaultHasher, Hash, Hasher},
  marker::PhantomData,
};

use num_traits::Euclid;

pub struct BloomFilter<T> {
  bits: Box<[u64]>,
  hashes: usize,
  _marker: PhantomData<T>,
}

const BITS: usize = std::mem::size_of::<u64>();

fn hash<T: Hash>(item: &T, offset: usize) -> usize {
  let mut s = DefaultHasher::new();
  (0x1 << offset).hash(&mut s);
  item.hash(&mut s);
  s.finish() as usize
}

impl<T: Hash> BloomFilter<T> {
  pub fn new(bits: usize, hashes: usize) -> Self {
    assert!(hashes > 0, "must use at least 1 hash");
    assert!(hashes <= BITS, "too many hashes");
    Self {
      // changing the length of bits would invalidate the entire bloom filter
      bits: vec![0u64; bits.div_ceil(BITS)].into_boxed_slice(),
      hashes,
      _marker: PhantomData,
    }
  }

  pub fn num_hashes(&self) -> usize {
    self.hashes
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
    let item = item.borrow();
    let num_bits = self.num_bits();
    (0..self.hashes)
      .map(|x| hash(item, x))
      .map(|x| x % num_bits)
      .for_each(|idx| {
        // eprintln!("{}", idx);
        let (word, bit) = idx.div_rem_euclid(&BITS);
        self.bits[word] |= 0x1 << bit;
      })
  }

  pub fn has(&self, item: impl Borrow<T>) -> bool {
    let item = item.borrow();
    let num_bits = self.num_bits();
    (0..self.hashes)
      .map(|x| hash(item, x))
      .map(|x| x % num_bits)
      .all(|idx| {
        let (word, bit) = idx.div_rem_euclid(&BITS);
        self.bits[word] & (0x1 << bit) != 0
      })
  }

  pub fn optimal(expected_items: usize, false_positive_rate: f64) -> Self {
    // https://en.wikipedia.org/wiki/Bloom_filter#Optimal_number_of_hash_functions
    let hashes = -(false_positive_rate.ln()) / (2.0f64).ln();
    let bits = expected_items as f64 * -2.08 * false_positive_rate.ln();
    let hashes = (hashes as usize).clamp(0, BITS);
    let bits = (bits as usize).next_multiple_of(BITS);
    Self::new(bits, hashes)
  }

  pub fn approx_items(&self) -> usize {
    // https://en.wikipedia.org/wiki/Bloom_filter#Approximating_the_number_of_items_in_a_Bloom_filter
    let k = self.hashes as f64;
    let m = self.num_bits() as f64;
    let x = self.num_set_bits() as f64;
    let res = -(m / k) * (1.0 - (x / m)).ln();
    res.round() as usize
  }

  pub fn false_positive_chance(&self) -> f64 {
    // https://en.wikipedia.org/wiki/Bloom_filter#Probability_of_false_positives
    let k = self.hashes as f64;
    let m = self.num_bits() as f64;
    let n = self.num_set_bits() as f64;

    let exp = (-k * n / m).exp();
    (1.0 - exp).powf(k)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_construction() {
    let mut bf: BloomFilter<u32> = BloomFilter::new(1024, 3);
    assert_eq!(bf.num_bits(), 1024);
    assert_eq!(bf.num_hashes(), 3);

    bf.insert(404);
    bf.insert(1013);
    bf.insert(234823);

    assert!(bf.has(404));
    assert!(bf.has(1013));
    assert!(bf.has(234823));
  }

  #[test]
  fn test_approx_items() {
    let num = 100;
    let bf = {
      let mut bf: BloomFilter<u32> = BloomFilter::new(1024, 4);
      for i in 0..num {
        bf.insert(i);
      }
      bf
    };

    let max_error = 0.1;
    let lower_limit = (num as f64 * (1.0 - max_error)) as usize;
    assert!(bf.approx_items() > lower_limit);
    let upper_limit = (num as f64 / (1.0 - max_error)) as usize;
    assert!(bf.approx_items() < upper_limit);
  }

  #[test]
  fn test_false_positive_chance() {
    let num = 100;
    let bf = {
      let mut bf: BloomFilter<u32> = BloomFilter::new(256, 4);
      for i in 0..num {
        bf.insert(i);
      }
      bf
    };

    let fpc_percent = (bf.false_positive_chance() * 100.0).round();
    assert_eq!(fpc_percent, 87.0);

    let num = 10;
    let bf = {
      let mut bf: BloomFilter<u32> = BloomFilter::new(256, 4);
      for i in 0..num {
        bf.insert(i);
      }
      bf
    };

    let fpc_percent = (bf.false_positive_chance() * 100.0).round();
    assert_eq!(fpc_percent, 4.0);
  }

  #[test]
  fn test_optimal() {
    let bf = BloomFilter::<u32>::optimal(10000, 0.1);
    assert_eq!(bf.num_bits(), 47896);
    assert_eq!(bf.num_hashes(), 3);

    let bf = BloomFilter::<u32>::optimal(1000, 0.01);
    assert_eq!(bf.num_bits(), 9584);
    assert_eq!(bf.num_hashes(), 6);

    let bf = BloomFilter::<u32>::optimal(20, 0.001);
    assert_eq!(bf.num_bits(), 288);
    assert_eq!(bf.num_hashes(), 8);
  }
}
