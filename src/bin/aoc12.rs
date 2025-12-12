use std::{fmt::Display, str::FromStr};

use aoc25::{
  exts::duration::DurationExt,
  grid::Grid,
  time::{time, time_try},
};

const INPUT: &str = include_str!("data/12.txt");

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Shape {
  No,
  Yes,
}

impl Display for Shape {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Shape::No => write!(f, " "),
      Shape::Yes => write!(f, "#"),
    }
  }
}

impl From<char> for Shape {
  fn from(value: char) -> Self {
    match value {
      '#' => Self::Yes,
      _ => Self::No,
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
struct Presents {
  presents: Vec<Grid<Shape>>,
  areas: Vec<(usize, usize, Vec<usize>)>,
}

impl FromStr for Presents {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut regions: Vec<_> = s.split("\n\n").collect();

    let areas = regions.pop().unwrap();
    let areas = areas
      .split('\n')
      .flat_map(|a| {
        let (area, list) = a.split_once(": ")?;

        let (x, y) = area.split_once('x')?;
        let list = list.split_whitespace().flat_map(|x| x.parse()).collect();

        Some((x.parse().ok()?, y.parse().ok()?, list))
      })
      .collect();

    let presents = regions
      .iter()
      .map(|p| {
        let data: Vec<Vec<Shape>> = p
          .lines()
          .skip(1)
          .map(|l| l.chars().map(|c| c.into()).collect())
          .collect();
        Grid::from_rows(data).unwrap()
      })
      .collect();

    Ok(Self { presents, areas })
  }
}

fn fits(grid: &Grid<Shape>, present: &Grid<Shape>, row: usize, col: usize) -> Option<Grid<Shape>> {
  let fits = present
    .cells()
    .filter(|(_, _, c)| matches!(c, Shape::Yes))
    .all(|(x, y, _)| matches!(grid.get(row + x, col + y), Some(Shape::No)));

  if fits {
    let mut new = grid.clone();
    for (x, y, _) in present.cells().filter(|(_, _, c)| matches!(c, Shape::Yes)) {
      new.set(row + x, col + y, Shape::Yes);
    }
    Some(new)
  } else {
    None
  }
}

fn rotations(shape: &Grid<Shape>) -> Vec<Grid<Shape>> {
  let r0 = shape.clone();
  let r0f = r0.flip();
  let r1 = r0.rotate();
  let r1f = r1.flip();
  let r2 = r1.rotate();
  let r2f = r2.flip();
  let r3 = r2.rotate();
  let r3f = r3.flip();
  vec![r0, r0f, r1, r1f, r2, r2f, r3, r3f]
}

fn can_fit_all(grid: Grid<Shape>, shapes: &[Grid<Shape>]) -> bool {
  if shapes.is_empty() {
    eprintln!("all shapes fit\n{grid}");
    return true;
  }

  for shape in rotations(&shapes[0]) {
    eprintln!("trying:\n{shape}\nin\n{grid}");
    for (x, y, _) in grid.cells() {
      eprintln!("trying at {x},{y}");
      let new = fits(&grid, &shape, x, y);
      if new.is_none() {
        continue;
      }
      let new = new.unwrap();
      eprintln!("fit at {x},{y}:\n{new}\n---");
      if can_fit_all(new, &shapes[1..]) {
        return true;
      }
    }
  }
  // eprintln!("failed\n{grid}");
  false
}

fn part_one(presents: &Presents) -> u64 {
  //

  let mut works = 0;
  for (x, y, counts) in &presents.areas {
    let grid = Grid::new(*x, *y, Shape::No);

    let shapes: Vec<Grid<Shape>> = counts
      .iter()
      .enumerate()
      .flat_map(|(index, count)| vec![presents.presents[index].clone(); *count])
      .collect();

    eprintln!("{}", shapes.len());

    if can_fit_all(grid, shapes.as_slice()) {
      works += 1;
    }
  }
  works
}

fn part_two(_presents: Presents) -> u64 {
  //
  0
}

fn main() -> anyhow::Result<()> {
  println!("AoC Day 12: Christmas Tree Farm");
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

  const SAMPLE_INPUT: &str = "0:\n###\n##.\n##.\n\n1:\n###\n##.\n.##\n\n2:\n.##\n###\n##.\n\n3:\n##.\n###\n##.\n\n4:\n###\n#..\n###\n\n5:\n###\n.#.\n###\n\n4x4: 0 0 0 0 2 0\n12x5: 1 0 1 0 2 2\n12x5: 1 0 1 0 3 2";

  #[test]
  fn test_one() {
    let points = SAMPLE_INPUT.parse().unwrap();
    let total = part_one(&points);
    assert_eq!(total, 2);
  }

  #[test]
  fn test_two() {
    let machine = SAMPLE_INPUT.parse().unwrap();
    let total = part_two(machine);
    assert_eq!(total, 0);
  }
}
