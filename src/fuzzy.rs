use std::collections::BTreeSet;

pub struct Bigrams<'a>(BTreeSet<&'a [u8; 2]>);

impl<'a> From<&'a str> for Bigrams<'a> {
  fn from(value: &'a str) -> Self {
    Bigrams(value.as_bytes().array_windows::<2>().collect())
  }
}

pub trait Bigrammable<'a> {
  fn bigrams(&self) -> Bigrams<'a>;
}

impl<'a> Bigrammable<'a> for &'a str {
  fn bigrams(&self) -> Bigrams<'a> {
    Bigrams(self.as_bytes().array_windows::<2>().collect())
  }
}

pub fn dice_sorensen<'a, 'b>(a: &Bigrams<'a>, b: &Bigrams<'b>) -> Score {
  let Bigrams(a) = a;
  let Bigrams(b) = b;
  let score = match (a.len(), b.len()) {
    (0, 0) => 1.0,
    (0, _) => 0.0,
    (_, 0) => 0.0,
    _ => {
      let num = 2 * a.intersection(&b).count();
      let den = a.len() + b.len();
      num as f64 / den as f64
    }
  };
  Score(score)
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Score(f64);
impl Eq for Score {}
impl Ord for Score {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.0.total_cmp(&other.0)
  }
}

pub fn find_closest<'a, 'b, T, I>(
  needle: impl Into<Bigrams<'a>>,
  haystack: T,
) -> Option<(&'b I, f64)>
where
  T: AsRef<[I]>,
  I: Bigrammable<'b>,
{
  let needle = needle.into();
  haystack
    .as_ref()
    .iter()
    .max_by_key(|cand| dice_sorensen(&(**cand).bigrams(), &needle));

  return None;
  /*
   *
   .map(|best| {
    let score = dice_sorensen(*best, needle);
    (*best, score.0)
  })
   */
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_equal() {
    assert_eq!(
      dice_sorensen(&"identical".into(), &"identical".into()),
      Score(1.0)
    );
  }

  /*
   *
  #[test]
  fn test_empty() {
    assert_eq!(dice_sorensen("empty", ""), Score(0.0));
    assert_eq!(dice_sorensen("", ""), Score(1.0));
  }

  #[test]
  fn test_similar() {
    assert_eq!(dice_sorensen("identical", "identics"), Score(0.8));
  }

  #[test]
  fn test_distinct() {
    assert_eq!(dice_sorensen("identical", "disjunct"), Score(0.0));
  }

  #[test]
  fn test_repeating() {
    assert_eq!(dice_sorensen("abab", "abababab"), Score(1.0));
  }

  #[test]
  fn test_search() {
    let needle = "target";
    let haystack = vec!["tiger", "ticker", "target", "targeting"];
    assert_eq!(find_closest(needle, haystack), Some(("target", 1.0)))
  }
   */
}
