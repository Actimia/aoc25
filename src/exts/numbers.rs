pub trait UnsignedExt {
  fn ratio(self, denominator: Self) -> f64;

  /// Computes the greatest common divisor of `Self` and `rhs`.
  /// Panics if either number is 0.
  fn gcd(self, rhs: Self) -> Self;

  /// Computes the lowest common multiple of `Self` and `rhs`.
  /// Panics if either number is 0.
  fn lcm(self, rhs: Self) -> Self;

  /// Computes the binomial coefficient, "`self` over `k`". This number is equivalent to `(self!)/(k! * (self - k)!)`.
  /// This comes up in several fields of mathematics, including combinatorics, where it is the number of ways k elements can be selected from self total elements, if order is not important.
  ///
  /// Computing this number involves numbers much larger than the result (although not as large as by the naive factorial formula). For large inputs, this algorithm may result in overflow, even if the theoretical result would fit in the type.
  fn choose(self, num: Self) -> Self;
}

impl UnsignedExt for u64 {
  fn choose(self, k: Self) -> Self {
    let (num, den) = (0..k)
      .map(|i| (self - i, i + 1))
      .fold((1, 1), |(num, den), (n, d)| {
        let num = num * n;
        let den = den * d;
        let gcd = num.gcd(den);
        (num / gcd, den / gcd)
      });
    num / den
  }

  fn ratio(self, denominator: Self) -> f64 {
    self as f64 / denominator as f64
  }

  fn gcd(self, rhs: Self) -> Self {
    // Euclid's algorithm: https://en.wikipedia.org/wiki/Euclidean_algorithm
    let mut a = self;
    let mut b = rhs;
    while b != 0 {
      (a, b) = (b, a % b);
    }
    a
  }

  fn lcm(self, rhs: Self) -> Self {
    self * (rhs / self.gcd(rhs))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_choose() {
    assert_eq!(8.choose(4), 70);
    assert_eq!(5.choose(1), 5);
    assert_eq!(1.choose(1), 1);
    assert_eq!(100.choose(15), 253338471349988640);
  }

  #[test]
  fn test_gcd() {
    assert_eq!(6.gcd(9), 3);
    assert_eq!(1071.gcd(462), 21);
  }

  #[test]
  fn test_lcm() {
    assert_eq!(6.lcm(9), 18);
    assert_eq!(4.lcm(20), 20);
    assert_eq!(21.lcm(6), 42);
  }

  #[test]
  fn test_ratio() {
    assert_eq!(1.ratio(2), 0.5);
    assert_eq!(2.ratio(5), 0.4);
  }
}
