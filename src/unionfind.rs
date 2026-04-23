use std::{cell::Cell, collections::HashMap, ops::Index};

#[derive(Debug)]
struct Entry<T> {
  item: T,
  parent: Cell<Option<usize>>,
  size: usize,
}

impl<T> Entry<T> {
  fn new(item: T) -> Self {
    Self {
      item,
      parent: Cell::new(None),
      size: 1,
    }
  }

  fn join(&mut self, index: usize, other: &Self) -> usize {
    other.parent.set(Some(index));
    self.size += other.size;
    self.size
  }
}

#[derive(Debug)]
pub struct UnionFind<T> {
  entries: Vec<Entry<T>>,
  sets: usize,
}

impl<T> UnionFind<T> {
  pub fn new(items: impl IntoIterator<Item = T>) -> Self {
    let entries: Vec<_> = items.into_iter().map(Entry::new).collect();
    let sets = entries.len();
    Self { entries, sets }
  }

  pub fn len(&self) -> usize {
    self.entries.len()
  }

  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  pub fn sets(&self) -> usize {
    self.sets
  }

  /// Iterates over the set that the item at index belongs to. O(n).
  pub fn neighbors(&self, index: usize) -> impl Iterator<Item = &T> {
    let root = self.find(index);
    self
      .entries
      .iter()
      .enumerate()
      .filter(move |(i, _)| self.find(*i) == root)
      .map(|(_, e)| &e.item)
  }

  fn find(&self, index: usize) -> usize {
    let mut root = index;

    while let Some(parent) = &self.entries[root].parent.get() {
      root = *parent;
    }

    if root != index {
      self.entries[index].parent.set(Some(root));
    }

    root
  }

  /// Total number of elements connected to an element. O(1) amortized.
  pub fn size(&self, index: usize) -> usize {
    let root = self.find(index);
    self.entries[root].size
  }

  pub fn connected(&self, a: usize, b: usize) -> bool {
    self.find(a) == self.find(b)
  }

  /// Joins two elements together. O(1) amortized.
  /// Ok(usize) -> the two elements were successfully joined, with the new total size
  /// Err(usize) -> the two elements were already part of the same subset, with the given size
  pub fn join(&mut self, a: usize, b: usize) -> Result<usize, usize> {
    let a_index = self.find(a);
    let b_index = self.find(b);

    if a_index == b_index {
      let entry = &self.entries[a_index];
      return Err(entry.size);
    }

    let [a, b] = self
      .entries
      .get_disjoint_mut([a_index, b_index])
      .expect("trust me bro");

    // by joining to the larger set, we minimize the tree depth
    let size = if a.size >= b.size {
      a.join(a_index, b)
    } else {
      b.join(b_index, a)
    };
    self.sets -= 1;

    Ok(size)
  }

  /// Converts the forest into a list of its disjoint components. Each sublist is ordered in the same order as the original element list, but the order of the sublists is unspecified. O(n).
  pub fn components(self) -> Vec<Vec<T>>
  where
    T: Clone,
  {
    let mut sets: HashMap<usize, Vec<T>> = HashMap::new();
    for (index, entry) in self.entries.iter().enumerate() {
      let index = self.find(index);
      sets.entry(index).or_default().push(entry.item.clone());
    }

    sets.into_values().collect()
  }
}

impl<T> Index<usize> for UnionFind<T> {
  type Output = T;

  fn index(&self, index: usize) -> &Self::Output {
    &self.entries[index].item
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::BTreeSet;

  #[test]
  fn basics() {
    let mut uf = UnionFind::new(0..5);
    assert_eq!(uf.sets(), 5);
    assert_eq!(uf.join(0, 0), Err(1));
    assert_eq!(uf.join(0, 1), Ok(2));
    assert_eq!(uf.join(1, 1), Err(2));
    assert_eq!(uf.join(2, 3), Ok(2));
    assert_eq!(uf.join(3, 4), Ok(3));
    assert_eq!(uf.sets(), 2);
    assert_eq!(uf.join(1, 3), Ok(5));
    assert_eq!(uf.join(0, 4), Err(5));
    assert_eq!(uf.size(0), 5);
    assert_eq!(uf.sets(), 1);
  }

  #[test]
  fn neighbors() {
    let mut uf = UnionFind::new(0..5);
    let _ = uf.join(0, 1);
    let _ = uf.join(1, 2);
    let neighbors: BTreeSet<_> = uf.neighbors(0).collect();
    assert_eq!(neighbors, BTreeSet::from_iter([0, 1, 2].iter()));
  }

  #[test]
  fn components() {
    let mut uf = UnionFind::new(0..5);
    let _ = uf.join(0, 1);
    let _ = uf.join(1, 2);
    let _ = uf.join(3, 4);
    let sets = BTreeSet::from_iter(uf.components());

    assert_eq!(sets, BTreeSet::from_iter([vec![0, 1, 2], vec![3, 4]]));
  }
}
