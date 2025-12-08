use std::fmt::{self, Display, Formatter};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T: Copy + PartialEq> {
    data: Vec<T>,
    n_rows: usize,
    n_cols: usize,
}

impl<T: Copy + PartialEq> Matrix<T> {
    pub fn new(n_rows: usize, n_cols: usize, data: Vec<T>) -> Self {
        assert_eq!(
            data.len(),
            n_rows * n_cols,
            "data length does not match dimensions"
        );
        Self {
            data,
            n_rows,
            n_cols,
        }
    }

    pub fn rows(&self) -> usize {
        self.n_rows
    }

    pub fn cols(&self) -> usize {
        self.n_cols
    }
    
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.data.iter().copied()
    }

    pub fn neighborhood(&self, row: usize, col: usize, radius: usize) -> Option<Matrix<T>> {
        if row >= self.n_rows || col >= self.n_cols {
            return None;
        }

        let r0 = row.saturating_sub(radius);
        let r1 = (row + radius).min(self.n_rows - 1);
        let c0 = col.saturating_sub(radius);
        let c1 = (col + radius).min(self.n_cols - 1);

        let nr = r1 - r0 + 1;
        let nc = c1 - c0 + 1;
        
        let mut data = Vec::with_capacity(nr * nc);
        for r in r0..=r1 {
            let base = r * self.n_cols;
            for c in c0..=c1 {
                data.push(self.data[base + c]);
            }
        }

        Some(Matrix::new(nr, nc, data))
    }
}

impl<T: Copy + PartialEq> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.n_cols + col]
    }
}

impl<T: Copy + PartialEq> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row * self.n_cols + col]
    }
}

impl<T> Display for Matrix<T>
where
    T: Copy + PartialEq + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for r in 0..self.n_rows {
            for c in 0..self.n_cols {
                if c > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", self[(r, c)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Copy + PartialEq> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(v: Vec<Vec<T>>) -> Self {
        Matrix::new( v.len(), v.first().map(|r| r.len()).unwrap_or(0), v.into_iter().flatten().collect())
    }
}
