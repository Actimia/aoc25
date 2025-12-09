use std::{fmt::Display, time::Instant};

use crate::exts::duration::DurationExt;

pub mod bloomfilter;
pub mod events;
pub mod exts;
pub mod graph;
pub mod graph_algo;
pub mod grid;
pub mod seq;
pub mod seq3;
pub mod vex;

pub fn time_quiet<T, V>(name: &str, func: T) -> V
where
  T: FnOnce() -> V,
{
  let start = Instant::now();
  let result = func();
  println!("{} (in {})", name, start.elapsed().display());
  result
}

pub fn time<T, V>(name: &str, func: T) -> V
where
  T: FnOnce() -> V,
  V: Display,
{
  let start = Instant::now();
  let result = func();
  println!("{}: {} (in {})", name, result, start.elapsed().display());
  result
}
