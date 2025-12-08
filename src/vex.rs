use std::{
  array::{self},
  ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Vex<T, const DIM: usize>([T; DIM]);

impl<T, const D: usize> Vex<T, D> {
  pub fn new(vals: impl Into<[T; D]>) -> Self {
    Vex(vals.into())
  }
}

impl<T: Default + Copy, const D: usize> Default for Vex<T, D> {
  fn default() -> Self {
    Self([T::default(); D])
  }
}

impl<T: Add<Output = T> + Copy, const D: usize> Add for Vex<T, D> {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self(array::from_fn(|i| self.0[i] + rhs.0[i]))
  }
}

impl<T: Add<Output = T> + Copy, const D: usize> AddAssign for Vex<T, D> {
  fn add_assign(&mut self, rhs: Self) {
    for (x, rx) in self.0.iter_mut().zip(rhs.0) {
      *x = *x + rx
    }
  }
}

impl<T: Sub<Output = T> + Copy, const D: usize> Sub for Vex<T, D> {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    Self(array::from_fn(|i| self.0[i] - rhs.0[i]))
  }
}

impl<T: Sub<Output = T> + Copy, const D: usize> SubAssign for Vex<T, D> {
  fn sub_assign(&mut self, rhs: Self) {
    for (x, rx) in self.0.iter_mut().zip(rhs.0) {
      *x = *x - rx
    }
  }
}

impl<T: Mul<Output = T> + Copy, const D: usize> Mul<T> for Vex<T, D> {
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self(array::from_fn(|i| self.0[i] * rhs))
  }
}

impl<T: MulAssign + Copy, const D: usize> MulAssign<T> for Vex<T, D> {
  fn mul_assign(&mut self, rhs: T) {
    self.0.iter_mut().for_each(|x| *x *= rhs);
  }
}

impl<T: Div<Output = T> + Copy, const D: usize> Div<T> for Vex<T, D> {
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self(array::from_fn(|i| self.0[i] / rhs))
  }
}

impl<T: DivAssign + Copy, const D: usize> DivAssign<T> for Vex<T, D> {
  fn div_assign(&mut self, rhs: T) {
    self.0.iter_mut().for_each(|x| *x /= rhs);
  }
}

impl<const D: usize> Vex<f64, D> {
  pub fn length(&self) -> f64 {
    self.0.iter().map(|x| x * x).sum::<f64>().sqrt()
  }

  pub fn normalize(&mut self) -> &Self {
    *self /= self.length();
    self
  }
}

impl<const D: usize> Vex<u64, D> {
  pub fn length2(&self) -> u64 {
    self.0.iter().map(|x| x * x).sum::<u64>()
  }
}

impl<T: Copy> Vex<T, 3> {
  pub fn x(&self) -> T {
    self.0[0]
  }
  pub fn y(&self) -> T {
    self.0[1]
  }
  pub fn z(&self) -> T {
    self.0[2]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add() {
    let v1 = Vex([3, 2]);
    let v2 = Vex([5, 1]);

    assert_eq!(v1 + v2, Vex([8, 3]));
  }

  #[test]
  fn test_add_assign() {
    let mut v1 = Vex([3, 2]);
    v1 += Vex([5, 1]);

    assert_eq!(v1, Vex([8, 3]));
  }

  #[test]
  fn test_sub() {
    let v1 = Vex([3, 2]);
    let v2 = Vex([5, 1]);

    assert_eq!(v1 - v2, Vex([-2, 1]));
  }

  #[test]
  fn test_sub_assign() {
    let mut v1 = Vex([3, 2]);
    v1 -= Vex([5, 1]);
    assert_eq!(v1, Vex([-2, 1]));
  }

  #[test]
  fn test_mul() {
    let v1 = Vex([3, 2]);
    assert_eq!(v1 * 2, Vex([6, 4]));
  }

  #[test]
  fn test_mul_assign() {
    let mut v1 = Vex([3, 2]);
    v1 *= 2;
    assert_eq!(v1, Vex([6, 4]));
  }

  #[test]
  fn test_div() {
    let v1 = Vex([3, 2]);
    assert_eq!(v1 / 2, Vex([1, 1]));
  }

  #[test]
  fn test_div_assign() {
    let mut v1 = Vex([3, 2]);
    v1 /= 2;
    assert_eq!(v1, Vex([1, 1]));
  }

  #[test]
  fn test_length() {
    let v1 = Vex([3.0, 4.0]);
    assert_eq!(v1.length(), 5.0)
  }

  #[test]
  fn test_normalize() {
    let mut v1 = Vex([3.0, 4.0]);
    assert_eq!(v1.normalize().length(), 1.0)
  }
}
