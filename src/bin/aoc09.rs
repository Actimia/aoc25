use std::{fmt::Display, time::Instant};

use aoc25::{grid::Grid, vex::Vex};

const INPUT: &str = include_str!("data/09.txt");

fn parse(input: &str) -> anyhow::Result<Vec<Vex<i64, 2>>> {
  let nums: Vec<Vex<i64, 2>> = input
    .lines()
    .flat_map(|l| {
      l.split_once(',')
        .into_iter()
        .flat_map(|(x, y)| -> Result<_, anyhow::Error> { Ok(Vex::new([x.parse()?, y.parse()?])) })
    })
    .collect();
  Ok(nums)
}

fn compute_rect(a: &Vex<i64, 2>, b: &Vex<i64, 2>) -> u64 {
  let xdiff = a.0[0].abs_diff(b.0[0]) + 1;
  let ydiff = a.0[1].abs_diff(b.0[1]) + 1;
  xdiff * ydiff
}

fn part_one(points: &Vec<Vex<i64, 2>>) -> u64 {
  // 4777824480
  let mut largest = 0;
  for v1 in points {
    for v2 in points {
      if v1 == v2 {
        continue;
      }

      let area = compute_rect(v1, v2);
      if area > largest {
        //eprintln!("rect: {:?}, {:?} = {}", v1, v2, area);
        largest = area;
      }
    }
  }
  largest
}

fn add_line(grid: &mut Grid<Tile>, a: &Vex<i64, 2>, b: &Vex<i64, 2>) {
  let dir = *b - *a;
  if dir.0[0] == 0 {
    let x = a.0[0];
    let start = a.0[1];
    let dirsign = dir.0[1].signum();
    for y in 0..=(dir.0[1].abs()) {
      grid.set(x as usize, (start + (y * dirsign)) as usize, Tile::Edge);
    }
  } else if dir.0[1] == 0 {
    let y = a.0[1];
    let start = a.0[0];
    let dirsign = dir.0[0].signum();
    for x in 0..=(dir.0[0].abs()) {
      grid.set((start + (x * dirsign)) as usize, y as usize, Tile::Edge);
    }
  }
}

fn flood_fill(grid: &mut Grid<Tile>) {
  let mut queue = vec![(0, 0)];
  while let Some((x, y)) = queue.pop() {
    if let Some(Tile::Inside) = grid.get(x, y) {
      grid.set(x, y, Tile::Outside);
      queue.push((x.saturating_sub(1), y));
      queue.push((x + 1, y));
      queue.push((x, y + 1));
      queue.push((x, y.saturating_sub(1)));
    }
  }
}
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
  Outside,
  Inside,
  Edge,
}

impl Display for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Tile::Outside => write!(f, " "),
      Tile::Edge => write!(f, "#"),
      Tile::Inside => write!(f, "."),
    }
  }
}

fn in_polygon(grid: &Grid<Tile>, x1: i64, y1: i64, x2: i64, y2: i64) -> bool {
  if x1 == x2 {
    let x = x1;
    for y in y1.min(y2)..y1.max(y2) {
      if let Some(Tile::Outside) = grid.get(x as usize, y as usize) {
        return false;
      }
    }
    return true;
  } else if y1 == y2 {
    let y = y1;
    for x in x1.min(x2)..x1.max(x2) {
      if let Some(Tile::Outside) = grid.get(x as usize, y as usize) {
        return false;
      }
    }
    return true;
  } else {
    unreachable!()
  }
}

fn part_two(points: &Vec<Vex<i64, 2>>) -> u64 {
  // 1542119040
  let compressed_xs = {
    let mut xs: Vec<i64> = points.iter().map(|v| v.0[0]).collect();
    xs.sort();
    xs
  };
  let compressed_ys = {
    let mut ys: Vec<i64> = points.iter().map(|v| v.0[1]).collect();
    ys.sort();
    ys
  };

  eprintln!(
    "compressed ({}, {})",
    compressed_xs.len(),
    compressed_ys.len()
  );

  let compressed: Vec<Vex<i64, 2>> = points
    .iter()
    .map(|v| {
      Vex::new([
        compressed_xs.iter().position(|x| *x == v.0[0]).unwrap() as i64 + 1,
        compressed_ys.iter().position(|x| *x == v.0[1]).unwrap() as i64 + 1,
      ])
    })
    .collect();

  let uncompress = |v: &Vex<i64, 2>| {
    let x = compressed_xs[v.0[0] as usize - 1];
    let y = compressed_ys[v.0[1] as usize - 1];
    Vex::new([x, y])
  };

  let grid_x = compressed_xs.len() + 2;
  let grid_y = compressed_ys.len() + 2;

  let mut grid = Grid::new(grid_x, grid_y, Tile::Inside);

  for w in compressed.windows(2) {
    let a = w[0];
    let b = w[1];
    add_line(&mut grid, &a, &b);
  }
  add_line(
    &mut grid,
    compressed.first().unwrap(),
    compressed.last().unwrap(),
  );

  flood_fill(&mut grid);

  let mut largest = 0;
  for v1 in &compressed {
    for v2 in &compressed {
      if v1 == v2 {
        continue;
      }

      let unv1 = uncompress(v1);
      let unv2 = uncompress(&v2);
      let area = compute_rect(&unv1, &unv2);
      if area <= largest {
        continue;
      }

      let x1 = v1.0[0];
      let y1 = v1.0[1];
      let x2 = v2.0[0];
      let y2 = v2.0[1];

      // x1y1 -> x1y2 -> x2y2 -> x2y1 -> x1y1
      if !in_polygon(&grid, x1, y1, x1, y2) {
        continue;
      }
      if !in_polygon(&grid, x1, y2, x2, y2) {
        continue;
      }
      if !in_polygon(&grid, x2, y2, x2, y1) {
        continue;
      }
      if !in_polygon(&grid, x2, y1, x1, y1) {
        continue;
      }
      largest = area;
    }
  }
  largest
}

fn main() -> anyhow::Result<()> {
  let start = Instant::now();
  let points = parse(INPUT)?;
  println!("Parsed input in {}us", start.elapsed().as_micros());

  let start = Instant::now();
  let part_one = part_one(&points);
  println!("Part 1: {part_one} (in {}us)", start.elapsed().as_micros());

  let start = Instant::now();
  let part_two = part_two(&points);
  println!("Part 2: {part_two} (in {}us)", start.elapsed().as_micros());
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  const SAMPLE_INPUT: &str = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3\n";

  #[test]
  fn test_one() {
    let points = parse(SAMPLE_INPUT).unwrap();
    let total = part_one(&points);
    assert_eq!(total, 50);
  }

  #[test]
  fn test_two() {
    let points = parse(SAMPLE_INPUT).unwrap();
    let total = part_two(&points);
    assert_eq!(total, 24);
  }
}
