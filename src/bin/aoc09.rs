use std::fmt::Display;

use aoc25::{grid::Grid, time, time_quiet, vex::Vex};

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
  let xdiff = a.x().abs_diff(b.x()) + 1;
  let ydiff = a.y().abs_diff(b.y()) + 1;
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
  if dir.x() == 0 {
    let x = a.x() as usize;
    let start = a.y();
    let dirsign = dir.y().signum();
    for y in 0..=(dir.y().abs()) {
      grid.set(x, (start + (y * dirsign)) as usize, Tile::Edge);
    }
  } else if dir.y() == 0 {
    let y = a.y() as usize;
    let start = a.x();
    let dirsign = dir.x().signum();
    for x in 0..=(dir.x().abs()) {
      grid.set((start + (x * dirsign)) as usize, y, Tile::Edge);
    }
  }
}

fn flood_fill(grid: &mut Grid<Tile>) {
  // (0,0) is guaranteed to not be in the polygon due to padding
  let mut queue = vec![(0, 0)];
  while let Some((x, y)) = queue.pop() {
    if let Some(Tile::Inside) = grid.get(x, y) {
      grid.set(x, y, Tile::Outside);
      if x != 0 {
        queue.push((x - 1, y));
      }
      queue.push((x + 1, y));
      if y != 0 {
        queue.push((x, y - 1));
      }
      queue.push((x, y + 1));
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

struct Compressed {
  points: Vec<Vex<i64, 2>>,
  xs: Vec<i64>,
  ys: Vec<i64>,
}

impl Compressed {
  fn compress(original: &Vec<Vex<i64, 2>>) -> Self {
    let (mut xs, mut ys): (Vec<i64>, Vec<i64>) = original.iter().map(|v| (v.x(), v.y())).collect();
    xs.sort();
    ys.sort();

    let points: Vec<Vex<i64, 2>> = original
      .iter()
      .map(|v| {
        Vex::new([
          xs.iter().position(|x| *x == v.x()).unwrap() as i64 + 1,
          ys.iter().position(|x| *x == v.y()).unwrap() as i64 + 1,
        ])
      })
      .collect();

    Self { points, xs, ys }
  }

  fn uncompress(&self, vex: &Vex<i64, 2>) -> Vex<i64, 2> {
    let x = self.xs[vex.x() as usize - 1];
    let y = self.ys[vex.y() as usize - 1];
    Vex::new([x, y])
  }
}

fn part_two(points: &Vec<Vex<i64, 2>>) -> u64 {
  // 1542119040
  let compressed = Compressed::compress(points);

  let mut grid = Grid::new(
    compressed.xs.len() + 2,
    compressed.ys.len() + 2,
    Tile::Inside,
  );

  for w in compressed.points.windows(2) {
    add_line(&mut grid, &w[0], &w[1]);
  }
  add_line(
    &mut grid,
    compressed.points.first().unwrap(),
    compressed.points.last().unwrap(),
  );

  flood_fill(&mut grid);

  #[cfg(test)]
  eprintln!("{grid}");

  let mut largest = 0;
  for v1 in &compressed.points {
    for v2 in &compressed.points {
      if v1 == v2 {
        continue;
      }

      let area = compute_rect(&compressed.uncompress(v1), &compressed.uncompress(v2));
      if area <= largest {
        // Skip expensive polygon check if this cannot be a candidate
        continue;
      }

      // x1y1 -> x1y2 -> x2y2 -> x2y1 -> x1y1
      if in_polygon(&grid, v1.x(), v1.y(), v1.x(), v2.y())
        && in_polygon(&grid, v1.x(), v2.y(), v2.x(), v2.y())
        && in_polygon(&grid, v2.x(), v2.y(), v2.x(), v1.y())
        && in_polygon(&grid, v2.x(), v1.y(), v1.x(), v1.y())
      {
        largest = area;
      }
    }
  }
  largest
}

fn main() -> anyhow::Result<()> {
  let points = time_quiet("Parsed input", || parse(INPUT))?;
  time("Part 1", || part_one(&points));
  time("Part 2", || part_two(&points));
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

  #[test]
  fn from_online() {
    let points = parse("1,0\n3,0\n3,6\n16,6\n16,0\n18,0\n18,9\n13,9\n13,7\n6,7\n6,9\n1,9").unwrap();
    let total = part_two(&points);
    assert_eq!(total, 30);
  }
}
