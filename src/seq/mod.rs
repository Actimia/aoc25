use std::iter::Peekable;

use crate::exts::IteratorExt;

pub trait Sequence: Iterator<Item = i8> {}

impl<T: Iterator<Item = i8>> Sequence for T {}

#[derive(Clone, Debug, PartialEq, Eq)]
enum SeqToken {
  Repeat,
  Num(i8),
  SubSequence(Seq),
  Random { min: i8, max: i8 },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Seq {
  pattern: Vec<SeqToken>,
  index: usize,
  last: i8,
}

impl Seq {
  fn new(pattern: Vec<SeqToken>) -> Self {
    Self {
      pattern,
      index: 0,
      last: 0,
    }
  }

  pub fn rand(min: i8, max: i8) -> Self {
    Self::new(vec![SeqToken::Random {
      min: min.min(max),
      max: min.max(max),
    }])
  }

  pub fn add(self, rhs: impl Into<Seq>) -> impl Sequence {
    self.zip(rhs.into()).map(|(l, r)| l.wrapping_add(r))
  }

  pub fn sub(self, rhs: impl Into<Seq>) -> impl Sequence {
    self.zip(rhs.into()).map(|(l, r)| l.wrapping_sub(r))
  }

  pub fn max(self, rhs: impl Into<Seq>) -> impl Sequence {
    self.zip(rhs.into()).map(|(l, r)| l.max(r))
  }

  pub fn min(self, rhs: impl Into<Seq>) -> impl Sequence {
    self.zip(rhs.into()).map(|(l, r)| l.min(r))
  }

  pub fn clamp(self, min: i8, max: i8) -> impl Sequence {
    self.map(move |x| x.clamp(min, max))
  }

  pub fn slow(self, x: usize) -> impl Sequence {
    self.repeat_each(x)
  }
}

impl From<i8> for Seq {
  fn from(value: i8) -> Self {
    Self::new(vec![SeqToken::Num(value)])
  }
}

impl From<&[i8]> for Seq {
  fn from(value: &[i8]) -> Self {
    Self::new(value.into_iter().copied().map(SeqToken::Num).collect())
  }
}

impl From<Vec<i8>> for Seq {
  fn from(value: Vec<i8>) -> Self {
    value.as_slice().into()
  }
}

impl<const L: usize> From<[i8; L]> for Seq {
  fn from(value: [i8; L]) -> Self {
    value.as_slice().into()
  }
}

impl TryFrom<&str> for Seq {
  type Error = anyhow::Error;

  fn try_from(text: &str) -> Result<Self, Self::Error> {
    fn read_num<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> anyhow::Result<i8> {
      fn to_num(c: char) -> i8 {
        ((c as u8) - ('0' as u8)) as i8
      }
      let mut acc: i8 = 0;
      while let Some(c) = chars.peek() {
        let c = c.clone();
        if c.is_numeric() {
          chars.next();
          acc = acc
            .checked_mul(10)
            .ok_or(anyhow::anyhow!("overflow"))?
            .checked_add(to_num(c))
            .ok_or(anyhow::anyhow!("overflow"))?
        } else {
          break;
        }
      }
      Ok(acc)
    }

    fn parse<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> anyhow::Result<Vec<SeqToken>> {
      let mut res = vec![];
      while let Some(c) = chars.peek() {
        let c = c.clone();
        match c {
          ' ' => {
            chars.next();
          }
          '<' => {
            chars.next();
            res.push(SeqToken::SubSequence(Seq::new(parse(chars)?)))
          }
          '>' => {
            chars.next();
            break;
          }
          '-' => {
            chars.next();
            let next = chars.peek().ok_or(anyhow::anyhow!("expected number"))?;
            if !next.is_numeric() {
              anyhow::bail!("expected numeric, got: {}", next)
            }
            res.push(SeqToken::Num(-read_num(chars)?))
          }
          '0'..='9' => res.push(SeqToken::Num(read_num(chars)?)),
          '_' => {
            chars.next();
            res.push(SeqToken::Repeat)
          }
          tok => anyhow::bail!("unexpected token: {}", tok),
        }
      }
      Ok(res)
    }

    let mut chars = text.chars().peekable();
    let pattern = parse(&mut chars)?;

    Ok(Seq::new(pattern))
  }
}

impl Iterator for Seq {
  type Item = i8;

  fn next(&mut self) -> Option<Self::Item> {
    let index = {
      let index = self.index;
      self.index = (self.index + 1) % self.pattern.len();
      index
    };
    let res = match self.pattern.get_mut(index).unwrap() {
      SeqToken::Repeat => self.last,
      SeqToken::Num(num) => *num,
      SeqToken::SubSequence(pattern_seq) => pattern_seq.next().expect("infinite iterator"),
      SeqToken::Random { min, max } => rand::random_range(*min..=*max),
    };
    self.last = res;
    Some(res)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simple() {
    let seq1 = Seq::from(1);

    assert_eq!(seq1.take(5).collect::<Vec<_>>(), [1, 1, 1, 1, 1]);

    let seq2: Seq = Seq::from([1, 2, 3]);
    assert_eq!(
      seq2.take(10).collect::<Vec<_>>(),
      vec![1, 2, 3, 1, 2, 3, 1, 2, 3, 1]
    );
  }

  #[test]
  fn test_add() {
    let seq1 = Seq::from([1, 2, 3]);

    assert_eq!(
      seq1.clone().add([1, -1]).take(10).collect::<Vec<_>>(),
      [2, 1, 4, 0, 3, 2, 2, 1, 4, 0]
    );

    assert_eq!(
      seq1.add(Seq::from([1, -1])).take(10).collect::<Vec<_>>(),
      [2, 1, 4, 0, 3, 2, 2, 1, 4, 0]
    );
  }

  #[test]
  fn test_transpose() {
    let seq1 = Seq::from([1, 2, 3, -1, 0]);
    assert_eq!(
      seq1.add(-4).take(10).collect::<Vec<_>>(),
      [-3, -2, -1, -5, -4, -3, -2, -1, -5, -4]
    )
  }

  #[test]
  fn test_pattern_simple() {
    let seq1 = Seq::try_from("1 2 3").unwrap();
    assert_eq!(seq1.take(5).collect::<Vec<_>>(), [1, 2, 3, 1, 2])
  }

  #[test]
  fn test_pattern_repeat() {
    let seq1 = Seq::try_from("1 2 _").unwrap();
    assert_eq!(
      seq1.take(10).collect::<Vec<_>>(),
      [1, 2, 2, 1, 2, 2, 1, 2, 2, 1]
    );

    let seq2 = Seq::try_from("1 2__ ").unwrap();
    assert_eq!(
      seq2.take(10).collect::<Vec<_>>(),
      [1, 2, 2, 2, 1, 2, 2, 2, 1, 2]
    );
  }

  #[test]
  fn test_pattern_subsequence() {
    let seq1 = Seq::try_from("1 <2 3> 4").unwrap();
    assert_eq!(
      seq1.take(10).collect::<Vec<_>>(),
      [1, 2, 4, 1, 3, 4, 1, 2, 4, 1]
    );

    let seq2: Seq = Seq::try_from("<2 3>").unwrap();
    assert_eq!(
      seq2.take(10).collect::<Vec<_>>(),
      [2, 3, 2, 3, 2, 3, 2, 3, 2, 3]
    );
  }

  #[test]
  fn test_pattern_subsequence_nested() {
    let seq1 = Seq::try_from("<2 1> <2 <3 5>>").unwrap();
    assert_eq!(
      seq1.take(10).collect::<Vec<_>>(),
      [2, 2, 1, 3, 2, 2, 1, 5, 2, 2]
    );
  }
}
