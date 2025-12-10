use std::{
  collections::{BTreeSet, VecDeque},
  str::FromStr,
};

use aoc25::{
  exts::duration::DurationExt,
  time::{time, time_try},
};
use glam::I64Vec2;

const INPUT: &str = include_str!("data/10.txt");

struct Machine {
  target: Vec<bool>,
  buttons: Vec<Vec<u32>>,
  joltage: Vec<u32>,
}

impl FromStr for Machine {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let split: Vec<&str> = s.split_whitespace().collect();
    let target = {
      let input = split.first().unwrap();
      input
        .chars()
        .flat_map(|c| match c {
          '.' => Some(false),
          '#' => Some(true),
          _ => None,
        })
        .collect()
    };

    let buttons: Vec<Vec<u32>> = split
      .iter()
      .filter(|s| s.starts_with('('))
      .map(|s| {
        s[1..s.len() - 1]
          .split(',')
          .flat_map(|n| n.parse())
          .collect::<Vec<u32>>()
      })
      .collect();

    let joltage = {
      let input = split.last().ok_or(anyhow::anyhow!("no joltage"))?;
      input[1..input.len() - 1]
        .split(',')
        .flat_map(|n| n.parse())
        .collect()
    };
    Ok(Self {
      target,
      buttons,
      joltage,
    })
  }
}

fn parse(input: &str) -> anyhow::Result<Vec<Machine>> {
  let problems = input.lines().flat_map(|l| l.parse()).collect();
  Ok(problems)
}

fn toggle(current: &Vec<bool>, button: &Vec<u32>) -> Vec<bool> {
  let mut new = current.clone();
  for x in button {
    new[*x as usize] ^= true;
  }
  new
}

fn search_buttons(initial: Vec<bool>, machine: &Machine) -> usize {
  let mut stack: VecDeque<(Vec<bool>, usize)> = VecDeque::new();
  stack.push_back((initial, 0));
  //search(initial, machine);

  while let Some((current, steps)) = stack.pop_back() {
    // eprintln!("{current:?} == {:?}", machine.target);
    if current == machine.target {
      return steps;
    }

    machine.buttons.iter().for_each(|buttons| {
      let toggled = toggle(&current, buttons);
      // eprintln!("{current:?} + {buttons:?} = {toggled:?}");
      stack.push_front((toggled, steps + 1))
    })
  }
  0
}

fn part_one(machines: &Vec<Machine>) -> usize {
  let mut total = 0;
  for (idx, machine) in machines.iter().enumerate() {
    let initial = vec![false; machine.target.len()];
    let steps = search_buttons(initial, machine);
    eprintln!("machine {idx} took {steps} steps");
    total += steps;
  }
  total
}

fn toggle_joltage(current: &Vec<u32>, button: &Vec<u32>) -> Vec<u32> {
  let mut new = current.clone();
  for index in button {
    new[*index as usize] += 1;
  }
  new
}

struct HeuristicCost(u32, Vec<u32>, usize);

impl HeuristicCost {
  fn from(steps: usize, vec: Vec<u32>, target: &Vec<u32>) -> Self {
    let diff = vec
      .iter()
      .zip(target.iter())
      .map(|(a, b)| a.abs_diff(*b))
      .sum();
    Self(diff, vec, steps)
  }
}

impl PartialEq for HeuristicCost {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}
impl Eq for HeuristicCost {}
impl PartialOrd for HeuristicCost {
  #[expect(clippy::non_canonical_partial_ord_impl)]
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.0.cmp(&other.0))
  }
}
impl Ord for HeuristicCost {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.0.cmp(&other.0)
  }
}

fn search_joltage(initial: Vec<u32>, machine: &Machine) -> usize {
  let mut heap: BTreeSet<HeuristicCost> = BTreeSet::new();
  heap.insert(HeuristicCost::from(0, initial, &machine.joltage));
  //search(initial, machine);
  let mut min_steps = usize::MAX;

  while let Some(HeuristicCost(_, current, steps)) = heap.pop_last() {
    if steps >= min_steps {
      eprintln!("abort due to steps");
      continue;
    }
    if current == machine.joltage {
      eprintln!("found in {steps}");
      min_steps = steps.min(min_steps);
      continue;
    }
    if current
      .iter()
      .zip(machine.joltage.iter())
      .any(|(a, b)| *a > *b)
    {
      eprintln!("abort due to too high {:?} {:?}", current, machine.joltage);
      continue;
    }

    machine.buttons.iter().for_each(|buttons| {
      let toggled = toggle_joltage(&current, buttons);
      eprintln!("{current:?} + {buttons:?} = {toggled:?}");
      heap.insert(HeuristicCost::from(steps + 1, toggled, &machine.joltage));
    })
  }
  min_steps
}

fn part_two(machines: &Vec<Machine>) -> usize {
  let mut total = 0;
  for (idx, machine) in machines.iter().enumerate() {
    let initial = vec![0; machine.joltage.len()];
    let steps = search_joltage(initial, machine);
    eprintln!("machine {idx} took {steps} steps");
    total += steps;
  }
  total
}

fn main() -> anyhow::Result<()> {
  let (machines, dur) = time_try(|| parse(INPUT))?;
  println!("Parsed points in {}", dur.display());

  let (part_one, dur) = time(|| part_one(&machines));
  println!("Part 1: {part_one} (in {})", dur.display());

  let (part_two, dur) = time(|| part_two(&machines));
  println!("Part 2: {part_two} (in {})", dur.display());

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
  /*
  #[test]
  fn test_one() {
    let points = parse(SAMPLE_INPUT).unwrap();
    let total = part_one(&points);
    assert_eq!(total, 7);
  }

  #[test]
  fn test_two() {
    let machine = parse(SAMPLE_INPUT).unwrap();
    let total = part_two(&machine);
    assert_eq!(total, 33);
  } */

  #[test]
  fn test_single() {
    let machine = parse("[.#...###] (2,3,4,5,6) (6,7) (0,1,3,5,6,7) (0,1,2,4,5,7) (1,3) (2,5) (1,2,4,5,6) (2,4,7) (1,4,5,6) {31,204,38,170,42,69,55,51}").unwrap();
    let total = part_two(&machine);
    assert_eq!(total, 33);
  }
}
