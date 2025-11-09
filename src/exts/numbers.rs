pub trait UnsignedExt {
  fn ratio(&self, denominator: Self) -> f64;
}

impl UnsignedExt for usize {
  fn ratio(&self, denominator: Self) -> f64 {
    *self as f64 / denominator as f64
  }
}
