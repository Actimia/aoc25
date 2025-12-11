use anyhow::bail;
use aoc25::{
  exts::duration::DurationExt,
  grid::Grid,
  time::{time, time_try},
};

const INPUT: &str = include_str!("data/04.txt");

#[derive(Clone, Copy, Eq, PartialEq)]
enum MapCell {
  None,
  Roll,
}

impl MapCell {
  fn is_roll(&self) -> bool {
    matches!(self, MapCell::Roll)
  }
}

impl TryFrom<char> for MapCell {
  type Error = anyhow::Error;

  fn try_from(value: char) -> Result<Self, Self::Error> {
    Ok(match value {
      '@' => MapCell::Roll,
      '.' => MapCell::None,
      _ => bail!("Unknown char"),
    })
  }
}

fn find_accessible(grid: &Grid<MapCell>) -> impl Iterator<Item = (usize, usize, &MapCell)> {
  const MAX_NEIGHBORS: usize = 3;

  grid
    .cells()
    .filter(|(_, _, c)| c.is_roll())
    .filter(|(row, col, _)| grid.count_neighbors(*row, *col, MapCell::is_roll) <= MAX_NEIGHBORS)
}

#[allow(unused)]
fn part_one(grid: &Grid<MapCell>) -> usize {
  // 1376
  find_accessible(grid).count()
}

#[allow(unused)]
fn part_two(grid: Grid<MapCell>) -> usize {
  // 8587
  fn remove_accessible(grid: Grid<MapCell>) -> (Grid<MapCell>, usize) {
    let mut next = grid.clone();
    let mut removed = 0;
    for (row, col, _) in find_accessible(&grid) {
      next[(row, col)] = MapCell::None;
      removed += 1
    }
    (next, removed)
  }

  let mut total_removed = 0;
  let mut grid = grid;
  loop {
    let (next, removed) = remove_accessible(grid);
    total_removed += removed;
    eprintln!("removed {removed} rolls");
    grid = next;
    if removed == 0 {
      break;
    }
  }
  total_removed
}

fn main() -> anyhow::Result<()> {
  println!("AoC Day 04: Printing Department");
  let (grid, dur) = time_try(|| Grid::from_str(INPUT))?;
  println!("Parsed input in {}", dur.display());

  let (part_one, dur) = time(|| part_one(&grid));
  println!("Part 1: {part_one} (in {})", dur.display());

  let (part_two, dur) = time(|| part_two(grid));
  println!("Part 2: {part_two} (in {})", dur.display());
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE_INPUT: &str = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

  #[test]
  fn test_part_one() {
    let grid = Grid::from_str(SAMPLE_INPUT).unwrap();
    let accessible = part_one(&grid);
    assert_eq!(accessible, 13);
  }

  #[test]
  fn test_part_two() {
    let grid = Grid::from_str(SAMPLE_INPUT).unwrap();
    let accessible = part_two(grid);
    assert_eq!(accessible, 43);
  }
}
