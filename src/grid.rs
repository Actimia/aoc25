use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Grid<T> {
    data: Vec<Vec<T>>, // Row wise
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for col in row {
                write!(f, "{} ", col)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl<T: Sized + Copy> Grid<T> {
    pub fn new(rows: usize, cols: usize, default: T) -> Self {
        Self {
            data: vec![vec![default; cols]; rows],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        let row = self.data.get(row)?;
        row.get(col)
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> T {
        let row = self.data.get_mut(row).unwrap();
        let prev = row[col];
        row[col] = value;
        prev
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

    pub fn neighbors_2<'a>(&'a self, row: usize, col: usize) -> impl Iterator<Item = &'a T> {
        self.data[(row - 1)..=(row + 1)]
            .iter()
            .flat_map(move |row| &row[(col - 1)..=(col + 1)])
    }

    pub fn neighbors_orthogonal(&self, row: usize, col: usize) -> impl Iterator<Item = &T> {
        let neighbors = [
            self.get(row - 1, col),
            self.get(row, col - 1),
            self.get(row, col + 1),
            self.get(row + 1, col),
        ];

        neighbors.into_iter().flatten()
    }
}

impl<'a, T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row][col]
    }
}

impl<'a, T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row][col]
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
}
