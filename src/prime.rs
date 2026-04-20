use std::ops::Shr;

use num_traits::{Bounded, Euclid, Num, One, Zero};
use rand::Rng;

/// Expresses an odd number n as (s,d), where n+1 = 2^s * d, with s as large as possible
fn highest_power(num: u64) -> (u64, u64) {
  if num == 1 {
    return (0, 0);
  }
  let n = num & !1; // turn into an even number
  for s in (1..=n.ilog2()).rev() {
    let x = 2u64.pow(s);
    let (d, rem) = n.div_rem_euclid(&x);
    if rem == 0 {
      return (s as u64, d);
    }
  }

  (0, num)
}

/// Checks whether num is prime using the Miller-Rabin test.
/// NB: This is a probabilistic primality test: a result of true means "probably prime",
/// while a result of false means "definitely composite". The chance of a false positive is at most 4^-k.
pub fn is_probably_prime(num: u64, k: usize) -> bool {
  let mut rng = rand::rng();
  let (s, d) = highest_power(num);

  for _ in 0..k {
    let a = rng.random_range(2..(num - 2));
    let mut x = power_mod(a as u128, d as u128, num as u128) as u64;
    let mut y = 0;
    for _ in 0..s {
      y = power_mod(x as u128, 2u128, num as u128) as u64;
      if y == 1 && x != 1 && x != num - 1 {
        return false;
      }
      x = y;
    }
    if y != 1 {
      return false;
    }
  }

  true
}

/// Checks whether num is prime using the Miller-Rabin test, with deterministic a's proven to be correct for all num < 2^64.
pub fn is_prime(num: u64) -> bool {
  let (s, d) = highest_power(num);

  const WITNESSES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
  for a in WITNESSES.into_iter().filter(|a| *a < num) {
    let mut x = power_mod(a as u128, d as u128, num as u128) as u64;
    let mut y = 0;
    for _ in 0..s {
      y = power_mod(x as u128, 2u128, num as u128) as u64;
      if y == 1 && x != 1 && x != num - 1 {
        return false;
      }
      x = y;
    }
    if y != 1 {
      return false;
    }
  }

  true
}

/// Fast modular exponentiation.
pub fn power_mod<A>(a: A, e: A, m: A) -> A
where
  A: Copy + PartialOrd + Num + Bounded,
  A: Shr<A, Output = A>,
{
  let zero: A = Zero::zero();
  let one: A = One::one();
  let two: A = one + one;
  let max: A = Bounded::max_value();
  assert!(m != zero);
  assert!((m - one) < (max / (m - one)));

  let mut result = one;
  let mut base = a % m;
  let mut exponent = e;

  loop {
    if exponent <= zero {
      break;
    }
    if exponent % two == one {
      result = (result * base) % m;
    }
    exponent = exponent >> one;
    base = (base * base) % m;
  }

  result
}

#[cfg(test)]
mod tests {
  use crate::prime::{highest_power, is_prime, is_probably_prime};

  #[test]
  fn miller_rabin() {
    assert_eq!(is_probably_prime(23, 5), true);
    assert_eq!(is_probably_prime(599, 5), true);
    assert_eq!(is_probably_prime(53270489, 5), true);
    assert_eq!(is_probably_prime(4294967291, 10), true); // 2^32 - 5
    assert_eq!(is_probably_prime(18446744073709551557, 25), true); // 2^64 - 59

    assert_eq!(is_probably_prime(15, 5), false);
    assert_eq!(is_probably_prime(4711, 5), false);
    assert_eq!(is_probably_prime(47114711, 5), false);
  }

  #[test]
  fn miller_rabin2() {
    assert_eq!(is_prime(23), true);
    assert_eq!(is_prime(599), true);
    assert_eq!(is_prime(53270489), true);
    assert_eq!(is_prime(4294967291), true); // 2^32 - 5
    assert_eq!(is_prime(18446744073709551557), true); // 2^64 - 59

    assert_eq!(is_prime(15), false);
    assert_eq!(is_prime(4711), false);
    assert_eq!(is_prime(47114711), false);
  }

  #[test]
  fn powers() {
    assert_eq!(highest_power(17), (4, 1));
    assert_eq!(highest_power(177), (4, 11));
    assert_eq!(highest_power(9), (3, 1));
    assert_eq!(highest_power(1001), (3, 125));
    assert_eq!(highest_power(4711), (1, 2355));
    assert_eq!(highest_power(53270493), (2, 13317623));
  }

  use quickcheck::TestResult;
  use quickcheck_macros::quickcheck;

  #[quickcheck]
  fn powers_parity(num: u64) -> TestResult {
    if num < 2 {
      return TestResult::discard();
    }

    let num = match num % 2 {
      0 => num + 1, // only applicable for odd numbers
      _ => num,
    };
    let (s, d) = highest_power(num);
    assert_eq!(num, (2u64.pow(s as u32) * d) + 1);
    assert_eq!(d % 2, 1);
    TestResult::passed()
  }
}
