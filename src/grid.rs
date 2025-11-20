use std::{
  fmt::{Debug, Display},
  ops::{Index, IndexMut},
};

use num_traits::Euclid;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid<T> {
  data: Box<[T]>,
  rows: usize,
  cols: usize,
}

impl<T: Display> Display for Grid<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for row in 0..self.rows {
      for col in 0..self.cols {
        write!(f, "{}", self[(row, col)])?;
      }
      writeln!(f)?
    }
    Ok(())
  }
}

impl<T: Sized + Copy> Grid<T> {
  pub fn new(rows: usize, cols: usize, default: T) -> Self {
    Self {
      data: vec![default; rows * cols].into_boxed_slice(),
      rows,
      cols,
    }
  }
}

impl<T> Grid<T> {
  /// Takes data stored in row-wise format and creates a Grid over it.
  /// Returns `None` if  `data.len()` is not a multiple of `cols`.
  pub fn from_data(data: Vec<T>, cols: usize) -> Option<Self> {
    let (rows, rem) = data.len().div_rem_euclid(&cols);
    if rem != 0 {
      return None;
    }
    Some(Self {
      data: data.into_boxed_slice(),
      rows,
      cols,
    })
  }

  pub fn rows(&self) -> usize {
    self.rows
  }

  pub fn cols(&self) -> usize {
    self.cols
  }

  /// Computes the actual data index of a coordinate pair. Returns None if the coordinates were invalid.
  fn get_index(&self, row: usize, col: usize) -> Option<usize> {
    if self.rows <= row || self.cols <= col {
      None
    } else {
      Some(row * self.cols + col)
    }
  }

  pub fn rotate_rows(&mut self, n: isize) {
    // push each row down n, filling with rows from the bottom
    let offset = n * self.cols as isize;

    if offset.is_positive() {
      self.data.rotate_right(offset as usize);
    } else {
      self.data.rotate_left((-offset) as usize);
    }
  }

  pub fn rotate_cols(&mut self, offset: isize) {
    // push each col right by n, filling with cols from the left

    for col in self.data.chunks_exact_mut(self.cols) {
      if offset.is_positive() {
        col.rotate_right(offset as usize);
      } else {
        col.rotate_left((-offset) as usize);
      }
    }
  }

  pub fn step(
    &self,
    from: (usize, usize),
    step: (isize, isize),
  ) -> impl Iterator<Item = (&T, (usize, usize))> {
    StepIterator {
      grid: self,
      cursor: from,
      step,
    }
  }

  pub fn iter_row(&self, row: usize) -> impl Iterator<Item = &T> {
    // let offset = self.get_index(row, 0)?.checked_sub(1).unwrap_or(0);
    // Some(self.data.iter().skip(offset).step_by(self.cols))
    self.step((row, 0), (0, 1)).map(|(x, _)| x)
  }

  pub fn iter_col(&self, col: usize) -> impl Iterator<Item = &T> {
    self.step((0, col), (1, 0)).map(|(x, _)| x)
  }

  /// Gets a reference to a cell in the grid. Returns None if the coordinates were invalid.
  pub fn get(&self, row: usize, col: usize) -> Option<&T> {
    let idx = self.get_index(row, col)?;
    self.data.get(idx)
  }

  /// Gets a mutable reference to a cell in the grid. Returns None if the coordinates were invalid.
  pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
    let idx = self.get_index(row, col)?;
    self.data.get_mut(idx)
  }

  /// Sets a cell in the grid, returning the previous value, or None if the coordinates were invalid.
  pub fn set(&mut self, row: usize, col: usize, value: T) -> Option<T> {
    let prev = self.get_mut(row, col)?;
    Some(std::mem::replace(prev, value))
  }

  pub fn neighbors(&self, row: usize, col: usize) -> impl Iterator<Item = &T> {
    let neighbors = [
      self.get(row - 1, col - 1),
      self.get(row - 1, col),
      self.get(row - 1, col + 1),
      self.get(row, col - 1),
      self.get(row, col + 1),
      self.get(row + 1, col - 1),
      self.get(row + 1, col),
      self.get(row + 1, col + 1),
    ];

    neighbors.into_iter().flatten()
  }

  pub fn orthogonal(&self, row: usize, col: usize) -> impl Iterator<Item = &T> {
    let neighbors = [
      self.get(row - 1, col),
      self.get(row, col - 1),
      self.get(row, col + 1),
      self.get(row + 1, col),
    ];

    neighbors.into_iter().flatten()
  }

  pub fn dimensions(&self) -> (usize, usize) {
    (self.rows, self.cols)
  }

  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self.data.iter()
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
    let row = self.cursor.0.checked_add_signed(self.step.0)?;
    let col = self.cursor.1.checked_add_signed(self.step.1)?;
    self.cursor = (row, col);
    self.grid.get(row, col).map(|value| (value, self.cursor))
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

    eprintln!("{}", grid);
    let steps: Vec<usize> = grid.step((0, 0), (0, 1)).map(|s| *s.0).collect();
    assert_eq!(steps, vec![1, 2, 3, 4]);

    let steps: Vec<usize> = grid.step((0, 0), (1, 0)).map(|s| *s.0).collect();
    assert_eq!(steps, vec![5, 10, 15, 20]);

    let steps: Vec<usize> = grid.step((0, 0), (1, 1)).map(|s| *s.0).collect();
    assert_eq!(steps, vec![6, 12, 18, 24]);
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
