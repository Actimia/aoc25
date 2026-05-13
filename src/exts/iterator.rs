use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub trait IteratorExt: Iterator + Sized {
  /// Removes duplicate items from the iterator by the specified key.
  fn unique_by<F, M>(self, mapper: F) -> UniqueIterator<Self, F, M>
  where
    Self: Sized,
    F: FnMut(&Self::Item) -> M,
    M: Hash + Eq,
  {
    UniqueIterator {
      inner: self,
      mapper,
      seen: HashSet::default(),
    }
  }

  /// Removes duplicate items from the iterator.
  fn unique(self) -> UniqueIterator<Self, impl FnMut(&Self::Item) -> Self::Item, Self::Item>
  where
    Self: Sized,
    Self::Item: Hash + Eq + Clone,
  {
    UniqueIterator {
      inner: self,
      mapper: |i| i.clone(),
      seen: HashSet::default(),
    }
  }

  fn repeat_each(self, count: usize) -> RepeatEachIterator<Self>
  where
    Self: Sized,
    Self::Item: Copy,
  {
    assert!(count != 0, "count cannot be 0");
    RepeatEachIterator {
      inner: self,
      count,
      next_item: None,
      cur: 0,
    }
  }

  fn flatten_verbose<T, E>(self) -> impl Iterator<Item = T>
  where
    Self: Sized,
    Self: Iterator<Item = Result<T, E>>,
    E: Debug,
  {
    VerboseFlatten { iter: self }
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

pub struct VerboseFlatten<I> {
  iter: I,
}

impl<I, T, E> Iterator for VerboseFlatten<I>
where
  I: Iterator<Item = Result<T, E>>,
  E: Debug,
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      match self.iter.next() {
        Some(r) => match r {
          Ok(item) => break Some(item),
          Err(e) => {
            eprintln!("{:?}", e);
            continue;
          }
        },
        None => break None,
      };
    }
  }
}

pub struct UniqueIterator<I: Iterator, F: FnMut(&I::Item) -> M, M: Hash + Eq> {
  inner: I,
  mapper: F,
  seen: HashSet<M>,
}

impl<I, F, M> Iterator for UniqueIterator<I, F, M>
where
  I: Iterator,
  F: FnMut(&I::Item) -> M,
  M: Hash + Eq,
{
  type Item = I::Item;

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
    let unique: Vec<_> = nums.into_iter().unique().collect();
    assert_eq!(unique, vec![1, 2, 3, 4]);
  }

  #[test]
  fn test_unique_by() {
    #[derive(PartialEq, Eq, Debug)]
    struct Val(i32, i32);

    let vals = vec![Val(1, 2), Val(1, 3), Val(2, 3), Val(1, 4)];
    let unique: Vec<_> = vals.into_iter().unique_by(|a| a.0).collect();
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
