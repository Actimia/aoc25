use std::{env, time::Duration};

use aoc25::seq::Seq;
use itertools::Itertools;

fn main() {
  let args: Vec<String> = env::args().collect();

  let mut patterns: Vec<Seq> = args[1..]
    .iter()
    .map(|p| Seq::try_from(p.as_str()).unwrap())
    .collect();

  let freq = {
    let tempo = 240f64;
    Duration::from_secs_f64(60.0 / tempo)
  };

  loop {
    let mut prev: usize = 0;
    println!();

    for note in patterns
      .iter_mut()
      .map(|pat| pat.next().unwrap())
      .map(|x| ((x + 30) as usize).clamp(0, 88))
      .unique()
      .sorted()
    {
      print!("{}█", " ".repeat(note - prev - 1));
      prev = note;
    }

    std::thread::sleep(freq);
  }
}
