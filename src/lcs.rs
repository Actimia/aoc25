use std::fmt::Debug;
use std::mem::swap;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Lcs {
  pos: usize,
  len: Option<usize>,
}
const ZERO: Lcs = Lcs { pos: 0, len: None };

pub fn longest_common_subsequence<'a, T: Eq>(left: &'a [T], right: &'a [T]) -> &'a [T] {
  let (long, short) = long_short(left, right);

  let mut cache = [vec![ZERO; short.len()], vec![ZERO; short.len()]];
  let [prev, cur] = cache.get_disjoint_mut([0, 1]).unwrap();

  fn get(xs: &[Lcs], i: isize) -> Lcs {
    if i < 0 { ZERO } else { xs[i as usize] }
  }

  for y in long {
    swap(cur, prev); // we swap the references only

    for (i, x) in short.iter().enumerate() {
      cur[i] = if x == y {
        let upleft = get(prev, i as isize - 1);
        Lcs {
          len: Some(upleft.len.unwrap_or(0) + 1),
          pos: upleft.pos,
        }
      } else {
        let left = get(cur, i as isize - 1);
        let up = get(prev, i as isize);
        if left.len >= up.len {
          left.clone()
        } else {
          up.clone()
        }
      };
    }
  }

  let res = cur.last().unwrap();
  &short[res.pos..(res.pos + res.len.unwrap_or(0))]
}

fn long_short<'a, T>(left: &'a [T], right: &'a [T]) -> (&'a [T], &'a [T]) {
  if left.len() >= right.len() {
    (left, right)
  } else {
    (right, left)
  }
}

/// Computes all the longest common substrings of two slices. O(m*n), where m and n are the lengths of the two slices.
pub fn longest_common_substrings<'a, T: Eq + Debug>(left: &'a [T], right: &'a [T]) -> Vec<&'a [T]> {
  let (long, short) = long_short(left, right);

  let mut suffix = vec![0; short.len()];
  let mut maxlen = 0;
  let mut result = Vec::default();

  for y in long {
    for (i, x) in short.iter().enumerate().rev() {
      suffix[i] = if x == y {
        let z = if i == 0 { 1 } else { suffix[i - 1] + 1 };
        if z > maxlen {
          maxlen = z;
          result.clear();
          result.push(i + 1); // end index is exclusive
        } else if z == maxlen {
          result.push(i + 1);
        }
        z
      } else {
        0
      };
    }
  }

  result
    .into_iter()
    .map(|last| &short[(last - maxlen)..last])
    .collect()
}

/// Finds the longest common substrings of two `str`s, in an UTF8-aware manner.
/// The length of the substrings are determined by their length in `char`s.
/// O(m*n), where m and n are the lengths of the strings.
pub fn longest_common_substrings_str<'a>(left: &'a str, right: &'a str) -> Vec<&'a str> {
  let mut maxlen_chars = 0;
  let mut maxlen_bytes = 0;
  let mut result = Vec::default();

  let rchars: Vec<_> = right.char_indices().collect();
  let mut suffix = vec![(0, 0); rchars.len()];

  for l in left.chars() {
    for (index_chars, (index_bytes, r)) in rchars.iter().enumerate().rev() {
      suffix[index_chars] = if *r == l {
        let len = r.len_utf8(); // byte length of char
        let (len_chars, len_bytes) = if index_chars == 0 {
          (1, len)
        } else {
          let (lc, lb) = suffix[index_chars - 1];
          (lc + 1, lb + len)
        };
        if len_chars > maxlen_chars {
          maxlen_chars = len_chars;
          maxlen_bytes = len_bytes;
          result.clear();
          result.push(index_bytes + len);
        } else if len_chars == maxlen_chars {
          result.push(index_bytes + len);
        }
        (len_chars, len_bytes)
      } else {
        (0, 0)
      };
    }
  }

  dbg!((&result, maxlen_bytes, maxlen_chars));
  result
    .into_iter()
    .map(|last| {
      right
        .get((last - maxlen_bytes)..last)
        .expect("should work lol")
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn subsequence_works() {
    let lcs = longest_common_subsequence(b"gac", b"agcat");
    assert_eq!(lcs, b"ga")
  }

  #[test]
  fn substring_works() {
    assert_eq!(longest_common_substrings(b"abcd", b"bcde"), &[b"bcd"]);
    assert_eq!(longest_common_substrings(b"abcd", b"abce"), &[b"abc"]);
    assert_eq!(
      longest_common_substrings(b"abcd", b"abceabc"),
      &[b"abc", b"abc"]
    );
    assert_eq!(longest_common_substrings(b"dabcd", b"eabce"), &[b"abc"]);
    assert_eq!(longest_common_substrings(b"test", b"fail"), &[] as &[&[u8]]);
    assert_eq!(longest_common_substrings(b"banana", b"ananas"), &[b"anana"]);
  }

  #[test]
  fn substring_str_works() {
    /*assert_eq!(longest_common_substrings_str("abcd", "bcde"), vec!["bcd"]);
    assert_eq!(longest_common_substrings_str("abcd", "abce"), vec!["abc"]);
    assert_eq!(
      longest_common_substrings_str("abcd", "abceabc"),
      vec!["abc", "abc"]
    );
    assert_eq!(longest_common_substrings_str("dabcd", "eabce"), vec!["abc"]);
    assert_eq!(
      longest_common_substrings_str("test", "fail"),
      vec![] as Vec<&str>
    );
    assert_eq!(
      longest_common_substrings_str("banana", "ananas"),
      vec!["anana"]
    );
    assert_eq!(
      longest_common_substrings_str("i am 👨", "i am 👩"),
      vec!["i am "]
    ); */
    assert_eq!(
      longest_common_substrings_str("a💙💙a💖💖a", "b💖💖b💙💙b"),
      vec!["💙💙", "💖💖",]
    );
  }
}
