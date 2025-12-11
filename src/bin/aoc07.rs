use std::mem;

use anyhow::bail;
use aoc25::{
  exts::duration::DurationExt,
  grid::Grid,
  time::{time, time_try},
};
use itertools::Itertools;

const INPUT: &str = include_str!("data/07.txt");

#[repr(u8)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
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
    .row(0)
    .map(|c| matches!(c, TachyonManifold::Start))
    .collect();

  let mut splits = 0;

  for row in 1..manifold.rows() {
    for splitter in manifold
      .row(row)
      .positions(|c| matches!(c, TachyonManifold::Splitter))
    {
      if beams[splitter] {
        splits += 1;
        beams[splitter] = false;
        // input is guaranteed to not have splitters by the edge of the grid
        beams[splitter - 1] = true;
        beams[splitter + 1] = true;
      }
    }
  }

  splits
}

#[allow(unused)]
fn part_two(manifold: &Grid<TachyonManifold>) -> u64 {
  // 431691375: too low
  // 16716444407407
  let mut timelines: Vec<u64> = manifold
    .row(0)
    .map(|c| match c {
      TachyonManifold::Start => 1,
      _ => 0,
    })
    .collect();

  for row in 1..manifold.rows() {
    for splitter in manifold
      .row(row)
      .positions(|c| matches!(c, TachyonManifold::Splitter))
    {
      if timelines[splitter] > 0 {
        let ways_here = mem::replace(&mut timelines[splitter], 0);
        // input is guaranteed to not have splitters by the edge of the grid
        timelines[splitter - 1] += ways_here;
        timelines[splitter + 1] += ways_here;
      }
    }
  }

  timelines.iter().sum()
}

fn main() -> anyhow::Result<()> {
  println!("AoC Day 07: Laboratories");
  let (manifold, dur) = time_try(|| Grid::from_str(INPUT))?;
  println!("Parsed input in {}", dur.display());

  let (part_one, dur) = time(|| part_one(&manifold));
  println!("Part 1: {part_one} (in {})", dur.display());

  let (part_two, dur) = time(|| part_two(&manifold));
  println!("Part 2: {part_two} (in {})", dur.display());
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
