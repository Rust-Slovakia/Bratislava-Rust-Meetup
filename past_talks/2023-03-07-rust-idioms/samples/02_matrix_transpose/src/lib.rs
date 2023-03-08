use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix {
    rows: Vec<Vec<isize>>,
    row_count: usize,
    col_count: usize,
}

impl Matrix {
    pub fn single_value(value: isize, rows: usize, cols: usize) -> Self {
        Self {
            rows: (0..rows)
                .map(|_| (0..cols).map(|_| value).collect())
                .collect(),
            row_count: rows,
            col_count: cols,
        }
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        Self::single_value(0, rows, cols)
    }

    pub fn identity(rows: usize, cols: usize) -> Self {
        Self {
            rows: (0..rows)
                .map(|r| (0..cols).map(|c| if r == c { 1 } else { 0 }).collect())
                .collect(),
            row_count: rows,
            col_count: cols,
        }
    }

    pub fn element_at(&self, row: usize, col: usize) -> isize {
        self.rows[row][col]
    }

    pub fn set_element_at(&mut self, row: usize, col: usize, value: isize) {
        self.rows[row][col] = value;
    }

    /// Create another matrix that is transposed version of this.
    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::zero(self.col_count, self.row_count);

        for row in 0..self.row_count {
            for col in 0..self.col_count {
                result.set_element_at(col, row, self.element_at(row, col));
            }
        }

        result
    }

    pub fn transpose_iterators(&self) -> Matrix {
        let rows = (0..self.col_count)
            .map(|ncol| {
                self.rows
                    .iter()
                    .map(|row| row[ncol])
                    .collect::<Vec<isize>>()
            })
            .collect();
        Matrix {
            rows,
            row_count: self.col_count,
            col_count: self.row_count,
        }
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        for row in &self.rows {
            write!(fmt, "(")?;
            for value in row {
                write!(fmt, " {value} ")?;
            }
            writeln!(fmt, ")")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn zero() {
        println!("{}", Matrix::zero(5, 5));
    }

    #[test]
    fn identity() {
        println!("{}", Matrix::identity(5, 5));
    }

    #[test]
    fn transpose_zero() {
        let zero = Matrix::zero(5, 5);
        let transp = zero.transpose();
        assert_eq!(zero, transp);
    }

    #[test]
    fn transpose_identity() {
        let identity = Matrix::identity(5, 5);
        let transp = identity.transpose();
        println!("{}", identity);
        println!("{}", transp);
        assert_eq!(identity, transp);
    }

    #[test]
    fn transpose_random() {
        let mut ran = Matrix::zero(5, 3);
        ran.set_element_at(0, 0, 0);
        ran.set_element_at(0, 1, 1);
        ran.set_element_at(0, 2, 2);

        ran.set_element_at(1, 0, 5);
        ran.set_element_at(1, 1, 6);
        ran.set_element_at(1, 2, 7);

        ran.set_element_at(2, 0, 10);
        ran.set_element_at(2, 1, 11);
        ran.set_element_at(2, 2, 12);

        ran.set_element_at(3, 0, 15);
        ran.set_element_at(3, 1, 16);
        ran.set_element_at(3, 2, 17);

        ran.set_element_at(4, 0, 18);
        ran.set_element_at(4, 1, 19);
        ran.set_element_at(4, 2, 20);
        println!("{}", ran);

        let transp = ran.transpose();
        println!("{}", transp);

        assert_eq!(ran, transp.transpose());
    }
}
