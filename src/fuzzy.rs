use std::collections::BTreeSet;

type Bigrams<'a> = BTreeSet<&'a [u8; 2]>;

pub fn dice_sorensen_str(a: &str, b: &str) -> Score {
  let a: Bigrams = a.as_bytes().array_windows::<2>().collect();
  let b: Bigrams = b.as_bytes().array_windows::<2>().collect();
  dice_sorensen(&a, &b)
}

pub fn dice_sorensen<T: Ord>(a: &BTreeSet<&'_ T>, b: &BTreeSet<&'_ T>) -> Score {
  let den = a.len() + b.len();
  if den == 0 {
    // Two empty sets are equal
    return Score(1.0);
  }
  let num = 2 * a.intersection(&b).count();
  Score(num as f64 / den as f64)
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Score(f64);

impl Eq for Score {}

impl Ord for Score {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.0.total_cmp(&other.0)
  }
}

pub fn dice_sorensen_search<'a, 'b>(
  needle: &'a str,
  haystack: impl AsRef<[&'b str]>,
) -> Option<(&'b str, f64)> {
  let needle: Bigrams = needle.as_bytes().array_windows::<2>().collect();
  haystack
    .as_ref()
    .iter()
    .max_by_key(|cand| {
      let cand: Bigrams = cand.as_bytes().array_windows::<2>().collect();
      dice_sorensen(&cand, &needle)
    })
    .map(|best| {
      let best_bigrams: Bigrams = best.as_bytes().array_windows::<2>().collect();
      let score = dice_sorensen(&best_bigrams, &needle);
      (*best, score.0)
    })
}

#[cfg(test)]
mod test {
  use super::*;
  use quickcheck_macros::quickcheck;

  #[quickcheck]
  fn is_symmetric(a: String, b: String) {
    assert_eq!(dice_sorensen_str(&a, &b), dice_sorensen_str(&b, &a))
  }

  #[test]
  fn test_equal() {
    assert_eq!(dice_sorensen_str("identical", "identical"), Score(1.0));
  }

  #[test]
  fn test_empty() {
    assert_eq!(dice_sorensen_str("empty", ""), Score(0.0));
    assert_eq!(dice_sorensen_str("", ""), Score(1.0));
  }

  #[test]
  fn test_similar() {
    assert_eq!(dice_sorensen_str("identical", "identics"), Score(0.8));
  }

  #[test]
  fn test_distinct() {
    assert_eq!(dice_sorensen_str("identical", "disjunct"), Score(0.0));
  }

  #[test]
  fn test_repeating() {
    assert_eq!(dice_sorensen_str("abab", "abababab"), Score(1.0));
  }

  #[test]
  fn test_search() {
    let needle = "target";
    let haystack = vec!["tiger", "ticker", "target", "targeting"];
    assert_eq!(
      dice_sorensen_search(needle, haystack),
      Some(("target", 1.0))
    )
  }
}
