use std::{
  collections::{HashMap, VecDeque},
  ops::Add,
  str::FromStr,
};

use aoc25::{
  exts::duration::DurationExt,
  time::{time, time_try},
};

const INPUT: &str = include_str!("data/11.txt");

struct Network(HashMap<String, Vec<String>>);

impl FromStr for Network {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut map = HashMap::new();
    for l in s.lines() {
      let (host, outputs) = l.split_once(':').ok_or(anyhow::anyhow!("expected :"))?;

      let outputs: Vec<_> = outputs
        .trim()
        .split_whitespace()
        .map(str::to_owned)
        .collect();

      let host = host.trim().to_owned();
      map.insert(host, outputs);
    }
    Ok(Self(map))
  }
}

fn part_one(Network(net): &Network) -> u64 {
  // 696
  let mut total = 0;

  let mut queue = VecDeque::new();
  queue.push_back("you");

  while let Some(node) = queue.pop_front() {
    if node == "out" {
      total += 1;
      continue;
    }

    if let Some(outputs) = net.get(node) {
      for output in outputs {
        queue.push_back(output.as_ref());
      }
    }
  }

  total
}

#[derive(Clone, Copy, Debug, Default)]
struct Counter {
  count: u64,
  dac: u64,
  fft: u64,
  both: u64,
}

impl Counter {
  fn new(count: u64) -> Self {
    Self {
      count,
      ..Self::default()
    }
  }
}

impl Add for Counter {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self {
      count: self.count + rhs.count,
      dac: self.dac + rhs.dac,
      fft: self.fft + rhs.fft,
      both: self.both + rhs.both,
    }
  }
}

fn part_two(net: Network) -> u64 {
  // 2844318424: too low
  // 6547319709817560: too high
  // 473741288064360
  fn count<'a>(net: &'a Network, node: &'a str, cache: &mut HashMap<&'a str, Counter>) -> Counter {
    if let Some(cached) = cache.get(node) {
      return *cached;
    }

    let mut counter = if let Some(neighbors) = net.0.get(node) {
      neighbors
        .iter()
        .map(|node| count(net, node, cache))
        .reduce(Counter::add)
        .unwrap()
    } else {
      // no neighbors => is out
      Counter::new(1)
    };

    if node == "dac" {
      counter.dac = counter.count;
      counter.both = counter.fft.min(counter.dac);
    } else if node == "fft" {
      counter.fft = counter.count;
      counter.both = counter.fft.min(counter.dac);
    }
    cache.insert(node, counter);
    counter
  }

  let res = count(&net, "svr", &mut HashMap::new());
  res.both
}

fn main() -> anyhow::Result<()> {
  println!("AoC Day 11: Reactor");
  let (network, dur) = time_try(|| INPUT.parse())?;
  println!("Parsed input in {}", dur.display());
  let (part_one, dur) = time(|| part_one(&network));
  println!("Part 1: {part_one} (in {})", dur.display());
  let (part_two, dur) = time(|| part_two(network));
  println!("Part 2: {part_two} (in {})", dur.display());
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE_INPUT: &str = "aaa: you hhh\nyou: bbb ccc\nbbb: ddd eee\nccc: ddd eee fff\nddd: ggg\neee: out\nfff: out\nggg: out\nhhh: ccc fff iii\niii: out";

  #[test]
  fn test_one() {
    let points = SAMPLE_INPUT.parse().unwrap();
    let total = part_one(&points);
    assert_eq!(total, 5);
  }

  const SAMPLE_INPUT_TWO: &str = "svr: aaa bbb\naaa: fft\nfft: ccc\nbbb: tty\ntty: ccc\nccc: ddd eee\nddd: hub\nhub: fff\neee: dac\ndac: fff\nfff: ggg hhh\nggg: out\nhhh: out";

  #[test]
  fn test_two() {
    let machine = SAMPLE_INPUT_TWO.parse().unwrap();
    let total = part_two(machine);
    assert_eq!(total, 2);
  }
}
