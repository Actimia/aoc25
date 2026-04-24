use std::fmt::Debug;

fn long_short<'a, T>(left: &'a [T], right: &'a [T]) -> (&'a [T], &'a [T]) {
  if left.len() >= right.len() {
    (left, right)
  } else {
    (right, left)
  }
}

/// Computes all the longest common substrings of two slices.
/// The order of the substrings returned is not specified.
///
/// Complexity: O(left.len() * right.len())
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
/// The order of the substrings returned is not specified.
///
/// Complexity: O(left.len() * right.len())
pub fn longest_common_substrings_str<'a>(left: &'a str, right: &'a str) -> Vec<&'a str> {
  let rchars: Vec<_> = right.char_indices().collect();

  let mut maxlen_chars = 0;
  let mut maxlen_bytes = 0;
  let mut suffix_cache = vec![(0, 0); rchars.len()];
  let mut result = Vec::default();

  for left_char in left.chars() {
    // By iterating backwards here, we only need a single cache line
    for (cache_index, (index_bytes, right_char)) in rchars.iter().enumerate().rev() {
      suffix_cache[cache_index] = if *right_char == left_char {
        let len = right_char.len_utf8(); // byte length of char
        let (len_chars, len_bytes) = if cache_index == 0 {
          (1, len)
        } else {
          let (lc, lb) = suffix_cache[cache_index - 1];
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
  fn substring() {
    assert_eq!(longest_common_substrings(b"abcd", b"bcde"), &[b"bcd"]);
    assert_eq!(longest_common_substrings(b"abcd", b"abce"), &[b"abc"]);
    assert_eq!(
      longest_common_substrings(b"abcd", b"abceabc"),
      &[b"abc", b"abc"]
    );
    assert_eq!(
      longest_common_substrings(b"a", b"aaa"),
      longest_common_substrings(b"aaa", b"a"),
    );
    assert_eq!(longest_common_substrings(b"dabcd", b"eabce"), &[b"abc"]);
    assert_eq!(longest_common_substrings(b"test", b"fail"), &[] as &[&[u8]]);
    assert_eq!(longest_common_substrings(b"banana", b"ananas"), &[b"anana"]);
  }

  #[test]
  fn substring_str() {
    assert_eq!(longest_common_substrings_str("abcd", "bcde"), vec!["bcd"]);
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
      longest_common_substrings_str("a", "aaa"),
      longest_common_substrings_str("aaa", "a"),
    );
    assert_eq!(
      longest_common_substrings_str("banana", "ananas"),
      vec!["anana"]
    );
    assert_eq!(
      longest_common_substrings_str("i am 👨", "i am 👩"),
      vec!["i am "]
    );
    assert_eq!(
      longest_common_substrings_str("a💙💙a💖💖a", "b💖💖b💙💙b"),
      vec!["💙💙", "💖💖",]
    );
  }
}
