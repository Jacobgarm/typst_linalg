use std::str::FromStr;

use crate::common::*;
use crate::convert::Convertable;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows: Vec<Vec<f64>>,
}

impl std::ops::Add for Matrix {
    type Output = Self;
    fn add(self, rhs: Matrix) -> Self::Output {
        let mut out = self.clone();
        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
                out[i][j] += rhs[i][j];
            }
        }
        out
    }
}

impl std::ops::Neg for Matrix {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut out = self.clone();
        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
                out[i][j] *= -1.0;
            }
        }
        out
    }
}

impl std::ops::Sub for Matrix {
    type Output = Self;
    fn sub(self, rhs: Matrix) -> Self::Output {
        let mut out = self.clone();
        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
                out[i][j] -= rhs[i][j];
            }
        }
        out
    }
}

impl std::ops::Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut out = Matrix::zero(self.nrows(), rhs.ncols());
        for i in 0..self.nrows() {
            for j in 0..rhs.ncols() {
                for k in 0..rhs.nrows() {
                    out[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        out
    }
}

impl std::ops::Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl std::ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}

#[allow(dead_code)]
impl Matrix {
    fn filled(rows: usize, cols: usize, value: f64) -> Self {
        let mut out: Vec<Vec<f64>> = Vec::new();
        let zero_vec = vec![value; cols];
        for _ in 0..rows {
            out.push(zero_vec.clone());
        }
        Matrix { rows: out }
    }

    fn zero(rows: usize, cols: usize) -> Self {
        Matrix::filled(rows, cols, 0.0)
    }

    fn id(dim: usize) -> Self {
        let mut out = Matrix::zero(dim, dim);
        for i in 0..dim {
            out[i][i] = 1.0;
        }
        out
    }

    fn nrows(&self) -> usize {
        self.rows.len()
    }

    fn ncols(&self) -> usize {
        self.rows[0].len()
    }

    fn is_square(&self) -> bool {
        self.nrows() == self.ncols()
    }

    pub fn transpose(&self) -> Self {
        let mut out = Matrix::zero(self.ncols(), self.nrows());
        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
                out[j][i] = self[i][j];
            }
        }
        out
    }

    fn scale(&self, scalar: f64) -> Self {
        let mut out = self.clone();
        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
                out[j][i] *= scalar;
            }
        }
        out
    }

    pub fn rowswap(&self, r1: usize, r2: usize) -> Result<Matrix, String> {
        if r1 >= self.nrows() || r2 >= self.nrows() {
            return Err(format!("Row index exceeds last row"));
        }
        let mut out = self.clone();
        out.rows.swap(r1, r2);
        Ok(out)
    }

    pub fn rowscale(&self, row: usize, c: f64) -> Result<Matrix, String> {
        if row >= self.nrows() {
            return Err(format!("Row index exceeds last row"));
        }
        let mut out = self.clone();
        out.rows[row] = out.rows[row].iter().map(|entry| entry * c).collect();
        Ok(out)
    }

    pub fn rowadd(&self, r1: usize, r2: usize, c: f64) -> Result<Matrix, String> {
        if r1 >= self.nrows() && r2 >= self.nrows() {
            return Err(format!("Row index exceeds last row"));
        }
        let mut out = self.clone();
        for i in 0..self.ncols() {
            out.rows[r1][i] += c * out.rows[r2][i];
        }
        Ok(out)
    }

    fn augment_cols(&self, right: Matrix) -> Result<Matrix, String> {
        if self.nrows() != right.nrows() {
            return Err("Cannot horizontally augment matrices of different heights".to_owned());
        }
        let mut augmented = self.clone();
        for i in 0..self.nrows() {
            augmented[i].extend(right[i].clone());
        }
        Ok(augmented)
    }

    fn augment_rows(&self, below: Matrix) -> Result<Matrix, String> {
        if self.ncols() != below.ncols() {
            return Err("Cannot vertically augment matrices of different widths".to_owned());
        }
        let mut augmented = self.clone();
        for i in 0..below.nrows() {
            augmented.rows.push(below[i].clone());
        }
        Ok(augmented)
    }

    pub fn det(&self) -> Result<f64, String> {
        if !self.is_square() {
            return Err("Non-square matrix has no determinant".to_owned());
        }
        let (mat_ref, swaps) = self.REF();
        let mut determinant = 1.0;
        for i in 0..self.ncols() {
            determinant *= mat_ref[i][i];
        }
        Ok((-1.0_f64).powi(swaps as i32) * determinant)
    }

    pub fn REF(&self) -> (Matrix, usize) {
        let mut out = self.clone();
        let rows = self.nrows();
        let cols = self.ncols();
        let mut prow = 0;
        let mut pcol = 0;
        let mut swaps = 0;

        while prow < rows && pcol < cols {
            let mut leading_values = vec![];
            let mut max_leading = prow;

            for i in prow..rows {
                if out.rows[i][pcol] != 0.0 {
                    leading_values.push(i);
                    dbg!(prow, i, out.rows[i][pcol], out.rows[max_leading][pcol]);
                    if out.rows[i][pcol].abs() > out.rows[max_leading][pcol].abs() {
                        max_leading = i;
                    }
                }
            }

            if leading_values.is_empty() {
                println!("continue");
                pcol += 1;
                continue;
            }
            if prow != max_leading {
                out = out.rowswap(prow, max_leading).unwrap();
                swaps += 1
            }

            for i in (prow + 1)..rows {
                let mult = out.rows[i][pcol] / out.rows[prow][pcol];
                out = out.rowadd(i, prow, -mult).unwrap();
                out.rows[i][pcol] = 0.0;
            }

            prow += 1;
            pcol += 1;
        }
        (out, swaps)
    }

    pub fn RREF(&self) -> Matrix {
        let (mut out, _) = self.REF();
        let rows = self.nrows();
        let cols = self.ncols();

        for i in 0..rows {
            let row = &out[i];
            if row.iter().all(|x| *x == 0.0) {
                return out;
            }
        }
        (out)
    }
}

impl Convertable for Matrix {
    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let s = std::str::from_utf8(bytes).unwrap().to_owned();

        let mut rows: Vec<Vec<f64>> = Vec::new();
        let mut row_length = usize::MAX;
        for row_str in s.split(';') {
            let mut row = Vec::new();
            for entry in row_str.split(',') {
                let res_float = f64::from_str(entry);
                match res_float {
                    Ok(float) => row.push(float),
                    Err(err) => return Err(err.to_string()),
                }
            }

            if row_length == usize::MAX {
                row_length = row.len();
            } else if row.len() != row_length {
                return Err("Non-rectangular matrix".to_owned());
            }

            rows.push(row);
        }
        Ok(Matrix { rows })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let s = self
            .rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|entry| {
                        let entry_s = entry.to_string();
                        truncate_zeroes(entry_s)
                    })
                    .collect::<Vec<String>>()
                    .join(",")
            })
            .collect::<Vec<String>>()
            .join(";");
        s.as_bytes().to_vec()
    }
}
