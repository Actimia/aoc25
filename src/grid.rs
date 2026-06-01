use std::{
  fmt::{Debug, Display},
  ops::{Index, IndexMut},
  str::FromStr,
};

use anyhow::ensure;
use itertools::Itertools;
use num_traits::Euclid;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid<T> {
  data: Box<[T]>,
  height: usize,
  width: usize,
}

impl<T: Display> Display for Grid<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for y in 0..self.height {
      for x in 0..self.width {
        write!(f, "{}", self[(x, y)])?;
      }
      writeln!(f)?
    }
    Ok(())
  }
}

impl<T: Sized + Copy> Grid<T> {
  pub fn new(width: usize, height: usize, default: T) -> Self {
    Self {
      data: vec![default; width * height].into_boxed_slice(),
      height,
      width,
    }
  }
}

impl<T> Grid<T> {
  /// Takes data stored in row-wise format and creates a Grid over it.
  /// Returns `Err` if  `data.len()` is not a multiple of `width`.
  pub fn from_data(data: impl IntoIterator<Item = T>, width: usize) -> anyhow::Result<Self> {
    let data: Vec<T> = data.into_iter().collect();
    let (rows, rem) = data.len().div_rem_euclid(&width);
    ensure!(rem == 0, "bad data size ({})", data.len());
    Ok(Self {
      data: data.into_boxed_slice(),
      height: rows,
      width,
    })
  }

  /// Constructs a grid from a Vec of rows, each of which is a Vec of data.
  /// Will give `err` if the outer `Vec` is empty, or if the inner `Vec`s do not have the same non-0 length.
  /// This is not the most hyperefficient way to parse this (`from_data` avoids additional allocations), but often maps nicer onto input data.
  pub fn from_rows(data: Vec<Vec<T>>) -> anyhow::Result<Self> {
    anyhow::ensure!(!data.is_empty(), "outer vec empty");
    let cols = data.first().unwrap().len();
    anyhow::ensure!(cols > 0, "inner vec empty");
    anyhow::ensure!(
      data.iter().map(Vec::len).all_equal(),
      "rows of unequal length"
    );

    Self::from_data(data.into_iter().flatten(), cols)
  }

  #[inline]
  pub fn height(&self) -> usize {
    self.height
  }

  #[inline]
  pub fn width(&self) -> usize {
    self.width
  }

  /// Computes the actual data index of a coordinate pair. Returns None if the coordinates are out of bounds.
  #[inline]
  fn get_index(&self, x: usize, y: usize) -> Option<usize> {
    if self.height <= y || self.width <= x {
      None
    } else {
      Some(y * self.width + x)
    }
  }

  /// Row-wise rotation of the grid.
  /// Pushes each row down by `n` (or up for negative `n`),
  /// filling with rows from the bottom (or top for negative `n`).
  pub fn rotate_rows(&mut self, n: isize) {
    let offset = n * self.width as isize;

    if offset.is_positive() {
      self.data.rotate_right(offset as usize);
    } else {
      self.data.rotate_left((-offset) as usize);
    }
  }

  /// Column-wise rotation of the grid.
  /// Pushes each column right by `n` (or left for negative `n`),
  /// filling with columns from the left (or right for negative `n`).
  pub fn rotate_cols(&mut self, offset: isize) {
    for col in self.data.chunks_exact_mut(self.width) {
      if offset.is_positive() {
        col.rotate_right(offset as usize);
      } else {
        col.rotate_left((-offset) as usize);
      }
    }
  }

  /// Steps through the grid until out of bounds.
  /// Panics if `from` is out of bounds or if `step == (0,0)`.
  pub fn step(
    &self,
    from: (usize, usize),
    step: (isize, isize),
  ) -> impl Iterator<Item = (&T, (usize, usize))> {
    assert_ne!(step, (0, 0));
    assert!(self.get_index(from.0, from.1).is_some());
    StepIterator {
      grid: self,
      cursor: from,
      step,
    }
  }

  pub fn row(&self, y: usize) -> impl DoubleEndedIterator<Item = &T> {
    let start = y * self.width();
    let end = start + self.width();
    self.data[start..end].iter()
  }

  pub fn row_mut(&mut self, y: usize) -> impl DoubleEndedIterator<Item = &mut T> {
    let start = y * self.width();
    let end = start + self.width();
    self.data[start..end].iter_mut()
  }

  pub fn col(&self, x: usize) -> impl DoubleEndedIterator<Item = &T> {
    let start = self.get_index(x, 0).unwrap();
    let step = self.width();
    self.data[start..].iter().step_by(step)
  }

  pub fn col_mut(&mut self, x: usize) -> impl DoubleEndedIterator<Item = &mut T> {
    let start = self.get_index(x, 0).unwrap();
    let step = self.width();
    self.data[start..].iter_mut().step_by(step)
  }

  /// Gets a reference to a cell in the grid. Returns None if the coordinates were invalid.
  pub fn get(&self, x: usize, y: usize) -> Option<&T> {
    let idx = self.get_index(x, y)?;
    self.data.get(idx)
  }

  /// Gets a mutable reference to a cell in the grid. Returns None if the coordinates were invalid.
  pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
    let idx = self.get_index(x, y)?;
    self.data.get_mut(idx)
  }

  /// Sets a cell in the grid, returning the previous value, or None if the coordinates were invalid.
  pub fn set(&mut self, x: usize, y: usize, value: T) -> Option<T> {
    let prev = self.get_mut(x, y)?;
    Some(std::mem::replace(prev, value))
  }

  pub fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = &T> {
    let neighbors = [
      self.get(x.wrapping_sub(1), y.wrapping_sub(1)),
      self.get(x, y.wrapping_sub(1)),
      self.get(x + 1, y.wrapping_sub(1)),
      self.get(x.wrapping_sub(1), y),
      self.get(x + 1, y),
      self.get(x.wrapping_sub(1), y + 1),
      self.get(x, y + 1),
      self.get(x + 1, y + 1),
    ];

    neighbors.into_iter().flatten()
  }

  pub fn orthogonal(&self, x: usize, y: usize) -> impl Iterator<Item = &T> {
    let neighbors = [
      self.get(x, y - 1),
      self.get(x - 1, y),
      self.get(x + 1, y),
      self.get(x, y + 1),
    ];

    neighbors.into_iter().flatten()
  }

  pub fn count_neighbors(&self, x: usize, y: usize, pred: impl Fn(&T) -> bool) -> usize {
    self.neighbors(x, y).filter(|v| pred(*v)).count()
  }

  pub fn dimensions(&self) -> (usize, usize) {
    (self.height, self.width)
  }

  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self.data.iter()
  }

  pub fn map<F, U>(self, f: F) -> Grid<U>
  where
    F: FnMut(T) -> U,
  {
    let data = self.data.into_iter().map(f);
    Grid::from_data(data, self.width).expect("From existing grid")
  }

  pub fn cells(&self) -> impl Iterator<Item = (usize, usize, &T)> {
    self.data.iter().enumerate().map(|(i, data)| {
      let (y, x) = i.div_rem_euclid(&self.width());
      (x, y, data)
    })
  }

  pub fn cells_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
    let rows = self.height();
    self.data.iter_mut().enumerate().map(move |(i, data)| {
      let (y, x) = i.div_rem_euclid(&rows);
      (x, y, data)
    })
  }

  pub fn transpose(&self) -> Self
  where
    T: Clone,
  {
    let data: Vec<T> = (0..self.width())
      .flat_map(|c| self.col(c).cloned())
      .collect();

    Grid::from_data(data, self.height()).unwrap()
  }

  pub fn rotate(&self) -> Self
  where
    T: Clone,
  {
    let data: Vec<_> = (0..self.width())
      .rev()
      .flat_map(|x| self.col(x))
      .cloned()
      .collect();

    Grid::from_data(data, self.height()).unwrap()
  }

  pub fn flip(&self) -> Self
  where
    T: Clone,
  {
    let data: Vec<_> = (0..self.height())
      .rev()
      .flat_map(|x| self.row(x).rev())
      .cloned()
      .collect();

    Grid::from_data(data, self.height()).unwrap()
  }
}

impl<T: TryFrom<char>> FromStr for Grid<T> {
  type Err = anyhow::Error;

  /// Constructs a grid from text, where each row of the text becomes a row of the grid,
  /// and each character is mapped to `T`.
  fn from_str(text: &str) -> anyhow::Result<Self> {
    let data = text
      .lines()
      .map(|l| l.chars().flat_map(|c| c.try_into()).collect())
      .collect();
    Self::from_rows(data)
  }
}

impl<T> IntoIterator for Grid<T> {
  type Item = T;

  type IntoIter = std::vec::IntoIter<T>;

  fn into_iter(self) -> Self::IntoIter {
    self.data.into_iter()
  }
}

struct StepIterator<'a, T> {
  grid: &'a Grid<T>,
  cursor: (usize, usize),
  step: (isize, isize),
}

impl<'a, T> Iterator for StepIterator<'a, T> {
  type Item = (&'a T, (usize, usize));

  fn next(&mut self) -> Option<Self::Item> {
    let res = self
      .grid
      .get(self.cursor.0, self.cursor.1)
      .map(|value| (value, self.cursor));
    self.cursor = (
      self.cursor.0.wrapping_add_signed(self.step.0),
      self.cursor.1.wrapping_add_signed(self.step.1),
    );
    res
  }
}

impl<T> Index<(usize, usize)> for Grid<T> {
  type Output = T;

  fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
    let idx = self.get_index(row, col).unwrap();
    &self.data[idx]
  }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
  fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
    let idx = self.get_index(row, col).unwrap();
    &mut self.data[idx]
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use core::assert_matches;

  #[test]
  fn test_construction() {
    let mut g = Grid::new(4, 4, 0.0);
    g[(0, 1)] = 4.0;

    assert_eq!(g.get(0, 0).unwrap(), &0.0);
    assert_eq!(g[(0, 1)], 4.0);
  }

  #[test]
  fn test_from_data() {
    let g = Grid::from_data(vec![1, 2, 3, 4, 5, 6], 3).unwrap();
    assert_eq!(g.dimensions(), (2, 3));
    assert_eq!(g[(1, 1)], 5);
    assert_eq!(g.get(4, 1), None);
  }

  #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
  enum TicTacToe {
    Empty,
    Naught,
    Cross,
  }

  impl Display for TicTacToe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        TicTacToe::Empty => write!(f, " "),
        TicTacToe::Naught => write!(f, "o"),
        TicTacToe::Cross => write!(f, "x"),
      }
    }
  }

  #[test]
  fn test_display() {
    let size = 4;
    let mut g = Grid::new(size, size, TicTacToe::Empty);
    for i in 0..size {
      g[(i, i)] = TicTacToe::Naught;
      g[(size - i - 1, i)] = TicTacToe::Cross;
    }

    let x = format!("{}", g);
    assert_eq!(x, "o  x\n ox \n xo \nx  o\n");
  }

  #[test]
  fn test_transpose() {
    let a: Grid<char> = Grid::from_str("abc\ndef").unwrap();
    let b = a.transpose();
    let transposed: Grid<char> = Grid::from_str("ad\nbe\ncf").unwrap();

    assert_eq!(a.width(), b.height());
    assert_eq!(a.height(), b.width());
    assert_eq!(b, transposed);
  }

  #[test]
  fn test_iter_row() {
    let grid: Grid<char> = Grid::from_str("1234\n4567\n7890").unwrap();
    assert_eq!(
      grid.row(0).copied().collect::<Vec<_>>(),
      vec!['1', '2', '3', '4']
    );
    assert_eq!(
      grid.row(1).copied().collect::<Vec<_>>(),
      vec!['4', '5', '6', '7']
    );
    assert_eq!(
      grid.row(2).copied().collect::<Vec<_>>(),
      vec!['7', '8', '9', '0']
    );
  }

  #[test]
  fn test_iter_col() {
    let grid: Grid<char> = Grid::from_str("1234\n4567\n7890").unwrap();
    assert_eq!(
      grid.col(0).copied().collect::<Vec<_>>(),
      vec!['1', '4', '7']
    );
    assert_eq!(
      grid.col(1).copied().collect::<Vec<_>>(),
      vec!['2', '5', '8']
    );
    assert_eq!(
      grid.col(2).copied().collect::<Vec<_>>(),
      vec!['3', '6', '9']
    );
    assert_eq!(
      grid.col(3).copied().collect::<Vec<_>>(),
      vec!['4', '7', '0']
    );
  }

  #[test]
  fn test_cells() {
    let g: Grid<char> = Grid::from_str("123\n456").unwrap();
    let coords: Vec<_> = g.cells().map(|(r, c, _)| (r, c)).collect();
    assert_eq!(coords, vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)]);
    let data: Vec<_> = g.cells().map(|(_, _, d)| d).copied().collect();
    assert_eq!(data, vec!['1', '2', '3', '4', '5', '6']);
  }

  #[test]
  fn test_steps() {
    let size = 5;
    let grid = {
      let mut g = Grid::new(size, size, 0);
      for r in 0..size {
        for c in 0..size {
          g[(r, c)] = (r * size) + c
        }
      }
      g
    };

    let steps: Vec<usize> = grid.step((0, 0), (0, 1)).map(|s| *s.0).collect();
    assert_eq!(steps, vec![0, 1, 2, 3, 4]);

    let steps: Vec<usize> = grid.step((0, 0), (1, 0)).map(|s| *s.0).collect();
    assert_eq!(steps, vec![0, 5, 10, 15, 20]);

    let steps: Vec<usize> = grid.step((0, 0), (1, 1)).map(|s| *s.0).collect();
    assert_eq!(steps, vec![0, 6, 12, 18, 24]);

    let steps: Vec<usize> = grid.step((4, 4), (0, -1)).map(|s| *s.0).collect();
    assert_eq!(steps, vec![24, 23, 22, 21, 20]);
  }

  #[test]
  fn test_rotate_rows() {
    let init = Grid::from_data(vec![1, 2, 3, 4, 5, 6], 2).unwrap();
    let mut grid = init.clone();

    let grid2 = Grid::from_data(vec![5, 6, 1, 2, 3, 4], 2).unwrap();

    grid.rotate_rows(1);
    assert_eq!(grid, grid2);

    let mut rotated1 = init.clone();
    let mut rotated2 = init.clone();
    rotated1.rotate_rows(2);
    rotated2.rotate_rows(-1);
    assert_eq!(rotated1, rotated2);
  }

  #[test]
  fn test_rotate_cols() {
    let init = Grid::from_data(vec![1, 2, 3, 4, 5, 6], 3).unwrap();
    let mut grid = init.clone();

    let grid2 = Grid::from_data(vec![3, 1, 2, 6, 4, 5], 3).unwrap();

    grid.rotate_cols(1);
    assert_eq!(grid, grid2);

    let mut rotated1 = init.clone();
    let mut rotated2 = init.clone();
    rotated1.rotate_cols(2);
    rotated2.rotate_cols(-1);
    assert_eq!(rotated1, rotated2);
  }
}
