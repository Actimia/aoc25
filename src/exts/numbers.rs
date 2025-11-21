pub trait UnsignedExt {
  fn ratio(self, denominator: Self) -> f64;

  fn gcd(self, rhs: Self) -> Self;

  fn lcm(self, rhs: Self) -> Self;
}

impl UnsignedExt for usize {
  fn ratio(self, denominator: Self) -> f64 {
    self as f64 / denominator as f64
  }

  /// Computes the greatest common divisor of `Self` and `rhs`.
  /// Panics if either number is 0.
  fn gcd(self, rhs: Self) -> Self {
    // Euclid's algorithm: https://en.wikipedia.org/wiki/Euclidean_algorithm
    let mut a = self;
    let mut b = rhs;
    while b != 0 {
      (a, b) = (b, a % b);
    }
    a
  }

  /// Computes the lowest common multiple of `Self` and `rhs`.
  /// Panics if either number is 0.
  fn lcm(self, rhs: Self) -> Self {
    self * (rhs / self.gcd(rhs))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
