use std::{cmp::Ordering, collections::BinaryHeap};

struct Priority<T>(usize, T);

impl<T> PartialEq for Priority<T> {
  fn eq(&self, other: &Self) -> bool {
    self.cmp(other) == Ordering::Equal
  }
}

impl<T> Eq for Priority<T> {}

impl<T> PartialOrd for Priority<T> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl<T> Ord for Priority<T> {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.0.cmp(&other.0)
  }
}

pub struct PriorityQueue<T> {
  heap: BinaryHeap<Priority<T>>,
}

impl<T> PriorityQueue<T> {
  pub fn new() -> Self {
    Self {
      heap: BinaryHeap::default(),
    }
  }

  pub fn push(&mut self, item: T, priority: usize) {
    self.heap.push(Priority(priority, item));
  }

  pub fn clear(&mut self) {
    self.heap.clear()
  }

  pub fn merge(&mut self, mut other: PriorityQueue<T>) {
    self.heap.append(&mut other.heap);
  }

  pub fn pop(&mut self) -> Option<T> {
    self.heap.pop().map(|x| x.1)
  }

  pub fn peek(&self) -> Option<&T> {
    self.heap.peek().map(|x| &x.1)
  }
}

impl<T> FromIterator<(usize, T)> for PriorityQueue<T> {
  fn from_iter<A: IntoIterator<Item = (usize, T)>>(iter: A) -> Self {
    Self {
      heap: BinaryHeap::from_iter(iter.into_iter().map(|(p, i)| Priority(p, i))),
    }
  }
}

pub struct PriorityQueueIterator<T> {
  heap: PriorityQueue<T>,
}

impl<T> Iterator for PriorityQueueIterator<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.heap.pop()
  }
}

impl<T> IntoIterator for PriorityQueue<T> {
  type Item = T;

  type IntoIter = PriorityQueueIterator<T>;

  fn into_iter(self) -> Self::IntoIter {
    PriorityQueueIterator { heap: self }
  }
}

#[cfg(test)]
mod tests {

  use itertools::Itertools;

  use super::*;

  #[test]
  fn basic() {
    let mut pq: PriorityQueue<&str> = PriorityQueue::new();
    pq.push("b", 5);
    pq.push("a", 10);

    assert_eq!(pq.pop(), Some("a"));
    assert_eq!(pq.pop(), Some("b"));
    assert_eq!(pq.pop(), None);
  }

  #[test]
  fn iter() {
    let mut pq: PriorityQueue<&str> = PriorityQueue::new();
    pq.push("hello", 22);
    pq.push("world", 10);

    assert_eq!(pq.into_iter().join(" "), "hello world")
  }
}
