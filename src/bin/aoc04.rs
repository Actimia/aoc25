use anyhow::bail;
use aoc25::grid::Grid;

const INPUT: &str = include_str!("data/04.txt");

#[derive(Clone, Copy, Eq, PartialEq)]
enum Map {
  None,
  Roll,
}

impl TryFrom<char> for Map {
  type Error = anyhow::Error;

  fn try_from(value: char) -> Result<Self, Self::Error> {
    Ok(match value {
      '@' => Map::Roll,
      '.' => Map::None,
      _ => bail!("Unknown char"),
    })
  }
}

fn parse_grid(input: &str) -> Grid<Map> {
  let lines: Vec<&str> = input.lines().collect();
  let rows = lines.len();
  let data: Vec<Map> = lines
    .iter()
    .map(|l| l.chars().flat_map(Map::try_from))
    .flatten()
    .collect();

  eprintln!("{}", data.len());

  let cols = data.len() / rows;
  eprintln!("{rows}x{cols}");
  Grid::from_data(data, cols).unwrap()
}

#[allow(unused)]
fn part_one(grid: Grid<Map>) -> u64 {
  let mut accessible = 0u64;
  for (row, col, cell) in grid.cells() {
    if !matches!(cell, Map::Roll) {
      continue;
    }
    let neighbor_rolls = grid
      .neighbors(row, col)
      .filter(|c| matches!(c, Map::Roll))
      .count();
    if neighbor_rolls < 4 {
      accessible += 1;
    }
  }
  accessible
}

#[allow(unused)]
fn part_two(grid: Grid<Map>) -> u64 {
  let mut accessible = 0u64;

  fn step(grid: &Grid<Map>) -> (Grid<Map>, u64) {
    let mut next = grid.clone();
    let mut num_removed = 0u64;
    for (row, col, data) in grid.cells().filter(|(_, _, c)| **c == Map::Roll) {
      let neighbor_rolls = grid
        .neighbors(row, col)
        .filter(|c| matches!(c, Map::Roll))
        .count();
      if neighbor_rolls < 4 {
        next[(row, col)] = Map::None;
        num_removed += 1
      }
    }
    (next, num_removed)
  }

  let mut total = 0u64;
  let mut grid = grid;
  loop {
    let (next, removed) = step(&grid);
    total += removed;
    grid = next;
    if removed == 0 {
      break;
    }
  }
  total
}

fn main() -> anyhow::Result<()> {
  let grid = parse_grid(INPUT);

  let accessible = part_two(grid);

  println!("{accessible}");
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_one() {
    let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
    let grid = parse_grid(input);
    let accessible = part_one(grid);
    assert_eq!(accessible, 13);
  }

  #[test]
  fn test_two() {
    let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
    let grid = parse_grid(input);
    let accessible = part_two(grid);
    assert_eq!(accessible, 43);
  }
}
