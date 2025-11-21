use std::ops::Add;

use crate::exts::UnsignedExt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sequence {
  Silence,
  Note(i8),
  Pattern {
    pattern: Vec<Sequence>,
    index: usize,
  },
  Add {
    left: Box<Sequence>,
    right: Box<Sequence>,
  },
  Random {
    min: i8,
    max: i8,
  },
}

impl Sequence {
  pub fn sample(self, n: usize) -> Vec<i8> {
    self.take(n).flatten().collect()
  }

  pub fn period(&self) -> Option<usize> {
    match self {
      Sequence::Silence => Some(1),
      Sequence::Note(_) => Some(1),
      Sequence::Pattern { pattern, index: _ } => {
        Some(pattern.iter().filter_map(|p| p.period()).sum())
      }
      Sequence::Add { left, right } => left.period().zip(right.period()).map(|(l, r)| l.lcm(r)),
      Sequence::Random { min: _, max: _ } => None,
    }
  }
}

impl Iterator for Sequence {
  type Item = Option<i8>;

  fn next(&mut self) -> Option<Self::Item> {
    let res = match self {
      Sequence::Silence => None,
      Sequence::Note(n) => Some(*n),
      Sequence::Pattern { pattern, index } => {
        let next = pattern.get_mut(*index)?.next();
        *index = (*index + 1) % pattern.len();
        next.flatten()
      }
      Sequence::Add { left, right } => left
        .next()
        .flatten()
        .zip(right.next().flatten())
        .map(|(l, r)| l + r),
      Sequence::Random { min, max } => Some(rand::random_range(*min..=*max)),
    };
    Some(res)
  }
}

impl<T: Into<Sequence>> Add<T> for Sequence {
  type Output = Sequence;

  fn add(self, rhs: T) -> Self::Output {
    Self::Add {
      left: self.into(),
      right: rhs.into().into(),
    }
  }
}

impl<R> From<R> for Sequence
where
  R: AsRef<[i8]>,
{
  fn from(value: R) -> Self {
    Self::Pattern {
      pattern: value.as_ref().iter().copied().map(Sequence::Note).collect(),
      index: 0,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simple() {
    let a: Sequence = [1, 2, 3].into();
    let b = a + [2, -1, 1, 0];

    assert_eq!(b.sample(10), vec![3, 1, 4, 1, 4, 2, 2, 2, 5, 0])
  }
}
