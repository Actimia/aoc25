use itertools::Itertools;

use crate::fuzzy::dice_sorensen_str;

/// Trait for loose comparisons.
///
/// Allows for checking whether two values are equal, within some tolerance. Exactly how this tolerance is defined is left up to each implementing type.
///
/// Implementations of this trait are not required to be symmetric (ie `a.eqish(b, t)` does not imply `b.eqish(a, t)`) or transitive (ie `a.eqish(b, t) == true` and `b.eqish(c, t)` does not imply `a.eqish(c, t)`). It is, however, required for implementations to be pure functions on their inputs.
pub trait Eqish<T = Self>: PartialEq {
  fn eqish(&self, other: &T, tolerance: f64) -> bool;
}

macro_rules! impl_num {
  ($type:ty) => {
    impl Eqish for $type {
      fn eqish(&self, other: &Self, tolerance: f64) -> bool {
        let a = *self as f64;
        let b = *other as f64;

        if a == 0.0 {
          return b.abs() < tolerance;
        }
        let abs_diff = a - b;
        abs_diff / a.abs() < tolerance
      }
    }
  };
}

impl_num!(u8);
impl_num!(u16);
impl_num!(u32);
impl_num!(u64);
impl_num!(u128);
impl_num!(usize);
impl_num!(i8);
impl_num!(i16);
impl_num!(i32);
impl_num!(i64);
impl_num!(i128);
impl_num!(isize);
impl_num!(f32);
impl_num!(f64);

impl Eqish for bool {
  fn eqish(&self, other: &Self, _tolerance: f64) -> bool {
    self == other
  }
}

impl<T: Eqish> Eqish for Vec<T> {
  fn eqish(&self, other: &Self, tolerance: f64) -> bool {
    let matches: usize = self
      .iter()
      .zip_longest(other)
      .map(|a| match a {
        itertools::EitherOrBoth::Both(a, b) => match a.eqish(b, tolerance) {
          true => 1,
          false => 0,
        },
        itertools::EitherOrBoth::Left(_) => 0,
        itertools::EitherOrBoth::Right(_) => 0,
      })
      .sum();

    matches.eqish(&self.len(), tolerance)
  }
}

impl Eqish for &str {
  fn eqish(&self, other: &Self, tolerance: f64) -> bool {
    // dice-sorensen is 1.0 for perfect equality, and 0 for complete inequality
    1.0 - dice_sorensen_str(self, other) < tolerance
  }
}

impl<T1: Eqish, T2: Eqish> Eqish for (T1, T2) {
  fn eqish(&self, other: &Self, tolerance: f64) -> bool {
    self.0.eqish(&other.0, tolerance) && self.1.eqish(&other.1, tolerance)
  }
}
impl<T1: Eqish, T2: Eqish, T3: Eqish> Eqish for (T1, T2, T3) {
  fn eqish(&self, other: &Self, tolerance: f64) -> bool {
    self.0.eqish(&other.0, tolerance)
      && self.1.eqish(&other.1, tolerance)
      && self.2.eqish(&other.2, tolerance)
  }
}
impl<T1: Eqish, T2: Eqish, T3: Eqish, T4: Eqish> Eqish for (T1, T2, T3, T4) {
  fn eqish(&self, other: &Self, tolerance: f64) -> bool {
    self.0.eqish(&other.0, tolerance)
      && self.1.eqish(&other.1, tolerance)
      && self.2.eqish(&other.2, tolerance)
      && self.3.eqish(&other.3, tolerance)
  }
}

/// Approximately compares two values (using the `Eqish` trait).
#[macro_export]
macro_rules! assert_eqish {
  ($left:expr, $right:expr $(,)?) => {
    assert_eqish!($left, $right, 0.01)
  };
  ($left:expr, $right:expr, $tol:expr) => {{
    let left = { ($left) };
    let right = { ($right) };
    let tolerance: f64 = ($tol);

    if !left.eqish(&right, tolerance) {
      panic!(
        "assertion `left ~= right` failed\n  left: {:?}\n right: {:?}",
        left, right
      );
    }
  }};
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_nums() {
    assert_eqish!(740u32, 741);
    assert_eqish!(32u8, 33, 0.1);
    assert_eqish!(3.141592, 3.14);
  }

  #[test]
  fn test_tuples() {
    assert_eqish!((410, 412), (410, 411));
    assert_eqish!((0.505, 1338), (0.5, 1337));
  }

  #[test]
  fn test_str() {
    assert_eqish!(
      "this is a comparison of two short texts",
      "this is a comparison of two short tests",
      0.1
    )
  }

  #[test]
  fn test_vec() {
    assert_eqish!(
      vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
      vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
    );
    assert_eqish!(
      vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
      vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
      0.2
    );
    assert_eqish!(
      vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
      vec![1, 2, 3, 4, 8, 6, 7, 8, 9, 0],
      0.2
    );
    assert_eqish!(
      vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
      vec![1, 2, 3, 4, 8, 6, 7, 8, 11],
      0.2
    );
  }
}
