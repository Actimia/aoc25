use num_traits::ConstZero;

pub trait Truthy {
  fn is_truthy(&self) -> bool;

  fn is_falsy(&self) -> bool {
    !self.is_truthy()
  }
}

macro_rules! impl_num {
  ($type:ty) => {
    impl Truthy for $type {
      fn is_truthy(&self) -> bool {
        *self == <$type>::ZERO
      }
    }
  };
}

impl_num!(u8);
impl_num!(i8);
impl_num!(u16);
impl_num!(i16);
impl_num!(u32);
impl_num!(i32);
impl_num!(u64);
impl_num!(i64);
impl_num!(u128);
impl_num!(i128);
impl_num!(usize);
impl_num!(isize);
impl_num!(f32);
impl_num!(f64);

impl Truthy for bool {
  fn is_truthy(&self) -> bool {
    *self
  }
}

impl<T> Truthy for Vec<T> {
  fn is_truthy(&self) -> bool {
    self.len().is_truthy()
  }
}
