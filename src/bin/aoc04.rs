use anyhow::bail;
use aoc25::grid::Grid;

const INPUT: &str = include_str!("data/04.txt");

#[derive(Clone, Copy, Eq, PartialEq)]
enum Map {
  None,
  Roll,
}

impl Map {
  fn is_roll(&self) -> bool {
    matches!(self, Map::Roll)
  }
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

  let cols = data.len() / rows;
  eprintln!("{rows}x{cols}");
  Grid::from_data(data, cols).unwrap()
}

fn find_accessible(grid: &Grid<Map>) -> impl Iterator<Item = (usize, usize, &Map)> {
  const MAX_NEIGHBORS: usize = 4;

  grid
    .cells()
    .filter(|(_, _, c)| c.is_roll())
    .filter(|(row, col, _)| grid.count_neighbors(*row, *col, Map::is_roll) < MAX_NEIGHBORS)
}

#[allow(unused)]
fn part_one(grid: Grid<Map>) -> usize {
  // 1376
  find_accessible(&grid).count()
}

#[allow(unused)]
fn part_two(grid: Grid<Map>) -> usize {
  // 8587
  fn remove_accessible(grid: &Grid<Map>) -> (Grid<Map>, usize) {
    let mut next = grid.clone();
    let mut removed = 0;
    for (row, col, _) in find_accessible(&grid) {
      next[(row, col)] = Map::None;
      removed += 1
    }
    (next, removed)
  }

  let mut total_removed = 0;
  let mut grid = grid;
  loop {
    let (next, removed) = remove_accessible(&grid);
    total_removed += removed;
    grid = next;
    if removed == 0 {
      break;
    }
  }
  total_removed
}

fn main() -> anyhow::Result<()> {
  let grid = parse_grid(INPUT);
  let part_one = part_one(grid.clone());
  println!("Part I: {part_one}");

  let part_two = part_two(grid);
  println!("Part II: {part_two}");
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
