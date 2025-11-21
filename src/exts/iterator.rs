use std::collections::HashSet;
use std::hash::Hash;

pub trait IteratorExt: Iterator + Sized {
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

  fn repeat_each(self, count: usize) -> RepeatEachIterator<Self>
  where
    Self::Item: Copy,
  {
    if count == 0 {
      panic!("count cannot be 0")
    }
    RepeatEachIterator {
      inner: self,
      count,
      next_item: None,
      cur: 0,
    }
  }
}

impl<T: Iterator> IteratorExt for T {}

pub struct RepeatEachIterator<T>
where
  T: Iterator,
  T::Item: Copy,
{
  inner: T,
  count: usize,
  next_item: Option<T::Item>,
  cur: usize,
}

impl<T> Iterator for RepeatEachIterator<T>
where
  T: Iterator,
  T::Item: Copy,
{
  type Item = T::Item;

  fn next(&mut self) -> Option<Self::Item> {
    if self.cur == self.count {
      self.next_item = None;
      self.cur = 0;
    }
    if self.next_item.is_none() {
      self.next_item = self.inner.next();
    }

    let item = self.next_item?;
    self.cur += 1;

    Some(item)
  }
}

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
    for next in self.inner.by_ref() {
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
    assert_eq!(unique, vec![1, 2, 3, 4]);
  }

  #[derive(PartialEq, Eq, Debug)]
  struct Val(i32, i32);

  #[test]
  fn test_unique_by() {
    let vals = vec![Val(1, 2), Val(1, 3), Val(2, 3), Val(1, 4)];
    let unique: Vec<_> = vals.into_iter().unique_by(|v| v.0).collect();
    assert_eq!(unique, vec![Val(1, 2), Val(2, 3)]);
  }

  #[test]
  fn test_repeat_each() {
    let vals = vec![1, 2, 3];
    let repeated: Vec<_> = vals.into_iter().repeat_each(3).collect();
    assert_eq!(repeated, vec![1, 1, 1, 2, 2, 2, 3, 3, 3]);
  }

  #[test]
  fn test_repeat_each_empty() {
    let vals: Vec<usize> = vec![];
    let repeated: Vec<_> = vals.into_iter().repeat_each(3).collect();
    assert_eq!(repeated, vec![]);
  }
}
