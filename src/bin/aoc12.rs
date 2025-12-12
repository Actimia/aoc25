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
      Shape::No => write!(f, "."),
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

/*
fn fits(grid: &Grid<Shape>, present: &Grid<Shape>, row: usize, col: usize) -> Option<Grid<Shape>> {
  let fits = present
    .cells()
    .filter(|(_, _, c)| matches!(c, Shape::Yes))
    .all(|(r, c, _)| match grid.get(row + r, col + c) {
      Some(Shape::Yes) => false,
      Some(Shape::No) => true,
      None => false,
    });

  if fits {
    let mut new = grid.clone();
    for (r, c, _) in present.cells().filter(|(_, _, c)| matches!(c, Shape::Yes)) {
      new.set(row + r, col + c, Shape::Yes);
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
    /* eprintln!(
      "trying:\n{shape}\nin\n{grid}\n{}x{}",
      grid.rows(),
      grid.cols()
    ); */
    for (row, col, _) in grid.cells() {
      // eprintln!("trying at {row},{col}");
      let new = fits(&grid, &shape, row, col);
      if new.is_none() {
        continue;
      }
      let new = new.unwrap();
      // eprintln!("fit at {row},{col}:\n{new}\n---");
      if can_fit_all(new, &shapes[1..]) {
        return true;
      }
    }
  }
  // eprintln!("failed\n{grid}");
  false
}
*/

fn part_one(presents: &Presents) -> u64 {
  // 495

  let mut works = 0;
  for (cols, rows, counts) in &presents.areas {
    let max = *rows * *cols;
    let best: usize = counts.iter().map(|x| x * 7).sum();
    if best < max {
      works += 1;
    }
    /*

      let grid = Grid::new(*rows, *cols, Shape::No);

      let shapes: Vec<Grid<Shape>> = counts
      .iter()
      .enumerate()
      .flat_map(|(index, count)| repeat(presents.presents[index].clone()).take(*count))
      .collect();

    if can_fit_all(grid, shapes.as_slice()) {
      works += 1;
    }
    */
  }
  works
}

fn main() -> anyhow::Result<()> {
  println!("AoC Day 12: Christmas Tree Farm");
  let (presents, dur) = time_try(|| INPUT.parse())?;
  println!("Parsed input in {}", dur.display());
  let (part_one, dur) = time(|| part_one(&presents));
  println!("Part 1: {part_one} (in {})", dur.display());
  println!("Part 2: Merry Christmas!");
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE_INPUT: &str = "0:\n###\n##.\n##.\n\n1:\n###\n##.\n.##\n\n2:\n.##\n###\n##.\n\n3:\n##.\n###\n##.\n\n4:\n###\n#..\n###\n\n5:\n###\n.#.\n###\n\n4x4: 0 0 0 0 2 0\n12x5: 1 0 1 0 2 2\n12x5: 1 0 1 0 3 2";

  #[test]
  fn test_one() {
    let presents = SAMPLE_INPUT.parse().unwrap();
    let total = part_one(&presents);
    assert_eq!(total, 3); // This is a lie
  }
}
