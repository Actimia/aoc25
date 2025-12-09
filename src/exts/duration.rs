use std::{fmt::Display, time::Duration};

pub trait DurationExt {
  fn display(&self) -> impl Display;
}

impl DurationExt for Duration {
  fn display(&self) -> impl Display {
    DurationDisplay(self)
  }
}

struct DurationDisplay<'a>(&'a Duration);

impl Display for DurationDisplay<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.0.as_secs_f64() > 0.5 {
      write!(f, "{:1}s", self.0.as_secs_f64())
    } else if self.0.as_millis() > 5 {
      write!(f, "{}ms", self.0.as_millis())
    } else {
      write!(f, "{}μs", self.0.as_micros())
    }
  }
}
