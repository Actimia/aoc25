use std::collections::HashSet;
use std::hash::Hash;

pub trait IteratorExt: Iterator {
  fn unique_by<M, F>(self, mapper: F) -> UniqueIterator<M, Self::Item, Self, F>
  where
    Self: Sized,
    F: FnMut(&Self::Item) -> M,
    M: Hash + Eq + Clone,
  {
    UniqueIterator {
      inner: self,
      mapper,
      seen: HashSet::default(),
    }
  }
}

impl<T: Iterator> IteratorExt for T {}

pub struct UniqueIterator<M: Hash + Eq + Clone, T, I: Iterator<Item = T>, F: FnMut(&T) -> M> {
  inner: I,
  mapper: F,
  seen: HashSet<M>,
}

impl<M: Hash + Eq + Clone, T, I: Iterator<Item = T>, F: FnMut(&T) -> M> Iterator
  for UniqueIterator<M, T, I, F>
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    while let Some(next) = self.inner.next() {
      let mapped = (self.mapper)(&next);
      if self.seen.contains(&mapped) {
        continue;
      }
      self.seen.insert(mapped);
      return Some(next);
    }
    None
  }
}

#[cfg(test)]
mod tests {
  use super::IteratorExt;

  #[test]
  fn test_unique() {
    let nums = vec![1, 2, 3, 3, 4, 1, 4];
    let unique: Vec<_> = nums.into_iter().unique_by(|&f| f.clone()).collect();
    assert_eq!(unique, vec![1, 2, 3, 4])
  }

  #[derive(PartialEq, Eq, Debug)]
  struct Val(i32, i32);

  #[test]
  fn test_unique_by() {
    let vals = vec![Val(1, 2), Val(1, 3), Val(2, 3), Val(1, 4)];
    let unique: Vec<_> = vals.into_iter().unique_by(|v| v.0).collect();
    assert_eq!(unique, vec![Val(1, 2), Val(2, 3)])
  }
}
