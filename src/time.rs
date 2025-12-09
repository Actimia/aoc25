use std::time::{Duration, Instant};

pub fn time<T, V>(func: T) -> (V, Duration)
where
  T: FnOnce() -> V,
{
  let start = Instant::now();
  let result = func();
  (result, start.elapsed())
}

pub trait Timing<T> {
  type Out;

  fn with_duration(self, duration: Duration) -> Self::Out;
}

impl<T> Timing<T> for Option<T> {
  type Out = Option<(T, Duration)>;

  fn with_duration(self, duration: Duration) -> Self::Out {
    self.map(|v| (v, duration))
  }
}

impl<T, E> Timing<T> for Result<T, E> {
  type Out = Result<(T, Duration), E>;

  fn with_duration(self, duration: Duration) -> Self::Out {
    self.map(|v| (v, duration))
  }
}

pub fn time_try<I, T>(func: fn() -> I) -> I::Out
where
  I: Timing<T>,
{
  let start = Instant::now();
  let result = func();
  result.with_duration(start.elapsed())
}
