use std::{
  fmt::Display,
  ops::{Index, IndexMut},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Grid<T> {
  data: Vec<T>,
  rows: usize,
  cols: usize,
}

impl<T: Display> Display for Grid<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for row in 0..self.rows {
      for col in 0..self.cols {
        write!(f, "{} ", self[(row, col)])?;
      }
      writeln!(f)?
    }
    Ok(())
  }
}

impl<T: Sized + Copy> Grid<T> {
  pub fn new(rows: usize, cols: usize, default: T) -> Self {
    Self {
      rows,
      cols,
      data: vec![default; rows * cols],
    }
  }
}
impl<T> Grid<T> {
  /// Computes the actual data index of a coordinate pair. Returns None if the coordinates were invalid.
  fn get_index(&self, row: usize, col: usize) -> Option<usize> {
    if self.rows <= row || self.cols <= col {
      None
    } else {
      Some(row * self.cols + col)
    }
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

  pub fn step(
    &self,
    from: (usize, usize),
    step: (isize, isize),
    wrapping: bool,
  ) -> impl Iterator<Item = (&T, (usize, usize))> {
    StepIterator {
      grid: self,
      cursor: from,
      step,
      wrapping,
    }
  }

  pub fn dimensions(&self) -> (usize, usize) {
    (self.rows, self.cols)
  }
}

struct StepIterator<'a, T> {
  grid: &'a Grid<T>,
  cursor: (usize, usize),
  step: (isize, isize),
  wrapping: bool,
}

impl<'a, T> Iterator for StepIterator<'a, T> {
  type Item = (&'a T, (usize, usize));

  fn next(&mut self) -> Option<Self::Item> {
    let row = if self.wrapping {
      self
        .step
        .0
        .checked_add_unsigned(self.cursor.0)?
        .rem_euclid(self.grid.rows as isize) as usize
    } else {
      self.cursor.0.checked_add_signed(self.step.0)?
    };
    let col = if self.wrapping {
      self
        .step
        .1
        .checked_add_unsigned(self.cursor.1)?
        .rem_euclid(self.grid.cols as isize) as usize
    } else {
      self.cursor.1.checked_add_signed(self.step.1)?
    };
    self.cursor = (row, col);
    self
      .grid
      .get(self.cursor.0, self.cursor.1)
      .map(|value| (value, self.cursor))
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

  #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
  enum TestValues {
    Empty,
    Naught,
    Cross,
  }

  impl Display for TestValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        TestValues::Empty => write!(f, " "),
        TestValues::Naught => write!(f, "o"),
        TestValues::Cross => write!(f, "x"),
      }
    }
  }

  #[test]
  fn test_display() {
    let size = 4;
    let mut g = Grid::new(size, size, TestValues::Empty);
    for i in 0..size {
      g[(i, i)] = TestValues::Naught;
      g[(size - i - 1, i)] = TestValues::Cross;
    }

    let x = format!("{}", g);
    assert_eq!(x, "o     x \n  o x   \n  x o   \nx     o \n");
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
    let steps: Vec<usize> = grid.step((0, 0), (0, 1), false).map(|s| *s.0).collect();
    assert_eq!(steps, vec![1, 2, 3, 4]);

    let steps: Vec<usize> = grid.step((0, 0), (1, 0), false).map(|s| *s.0).collect();
    assert_eq!(steps, vec![5, 10, 15, 20]);

    let steps: Vec<usize> = grid.step((0, 0), (1, 1), false).map(|s| *s.0).collect();
    assert_eq!(steps, vec![6, 12, 18, 24]);

    let steps: Vec<usize> = grid
      .step((0, 0), (2, -1), true)
      .take(6)
      .map(|s| *s.0)
      .collect();
    assert_eq!(steps, vec![14, 23, 7, 16, 0, 14]);
  }
}
