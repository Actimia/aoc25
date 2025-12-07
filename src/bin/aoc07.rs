use std::{mem, time::Instant};

use anyhow::bail;
use aoc25::grid::Grid;
use itertools::Itertools;

const INPUT: &str = include_str!("data/07.txt");

enum TachyonManifold {
  Empty,
  Splitter,
  Start,
}

impl TryFrom<char> for TachyonManifold {
  type Error = anyhow::Error;

  fn try_from(value: char) -> Result<Self, Self::Error> {
    Ok(match value {
      '.' => Self::Empty,
      '^' => Self::Splitter,
      'S' => Self::Start,
      c => bail!("unknown char ({c})"),
    })
  }
}

#[allow(unused)]
fn part_one(manifold: &Grid<TachyonManifold>) -> u64 {
  // 1585
  let mut beams: Vec<bool> = manifold
    .iter_row(0)
    .map(|c| matches!(c, TachyonManifold::Start))
    .collect();

  let mut splits = 0;

  for row in 1..manifold.rows() {
    let mut next_beams = beams.clone();
    for splitter in manifold
      .iter_row(row)
      .positions(|c| matches!(c, TachyonManifold::Splitter))
    {
      if beams[splitter] {
        splits += 1;
        next_beams[splitter] = false;
        if let Some(left) = next_beams.get_mut(splitter.wrapping_sub(1)) {
          *left = true;
        }
        if let Some(right) = next_beams.get_mut(splitter + 1) {
          *right = true;
        }
      }
    }
    beams = next_beams;
  }

  splits
}

#[allow(unused)]
fn part_two(manifold: &Grid<TachyonManifold>) -> u64 {
  // 431691375: too low
  // 16716444407407
  let mut beams: Vec<u64> = manifold
    .iter_row(0)
    .map(|c| match c {
      TachyonManifold::Start => 1,
      _ => 0,
    })
    .collect();

  for row in 1..manifold.rows() {
    for splitter in manifold
      .iter_row(row)
      .positions(|c| matches!(c, TachyonManifold::Splitter))
    {
      if beams[splitter] > 0 {
        let timelines_here = mem::replace(&mut beams[splitter], 0);
        if let Some(left) = beams.get_mut(splitter.wrapping_sub(1)) {
          *left += timelines_here;
        }
        if let Some(right) = beams.get_mut(splitter + 1) {
          *right += timelines_here;
        }
      }
    }
  }

  beams.iter().sum()
}

fn main() -> anyhow::Result<()> {
  let start = Instant::now();
  let manifold: Grid<TachyonManifold> = Grid::from_str(INPUT)?;
  println!("Parsed input in {}μs", start.elapsed().as_micros());

  let start = Instant::now();
  let part_one = part_one(&manifold);
  println!("Part 1: {part_one} (in {}μs)", start.elapsed().as_micros());

  let start = Instant::now();
  let part_two = part_two(&manifold);
  println!("Part 2: {part_two} (in {}μs)", start.elapsed().as_micros());
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE_INPUT: &str = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";

  #[test]
  fn test_one() {
    let manifold = Grid::from_str(SAMPLE_INPUT).unwrap();
    let splits = part_one(&manifold);
    assert_eq!(splits, 21);
  }

  #[test]
  fn test_two() {
    let manifold = Grid::from_str(SAMPLE_INPUT).unwrap();
    let timelines = part_two(&manifold);
    assert_eq!(timelines, 40);
  }
}
