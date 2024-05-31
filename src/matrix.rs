use std::ops::Neg;
use std::str::FromStr;

use crate::common::*;
use crate::convert::Convertable;
use crate::vector::Vector;

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

    fn is_invertible(&self) -> bool {
        self.is_square() && self.REF().0[self.nrows() - 1][self.ncols() - 1] != 0.0
    }

    pub fn is_symmetric(&self) -> bool {
        if !self.is_square() {
            return false;
        };
        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
                if i != j && self[i][j] != self[j][i] {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_skew_symmetric(&self) -> bool {
        if !self.is_square() {
            return false;
        };
        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
                if self[i][j] != -self[j][i] {
                    return false;
                }
            }
        }
        true
    }
    pub fn is_diagonal(&self) -> bool {
        if !self.is_square() {
            return false;
        };
        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
                if i != j && self[i][j] != 0.0 {
                    return false;
                }
            }
        }
        true
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
                out[i][j] *= scalar;
            }
        }
        out
    }

    fn hadamard(&self, rhs: Matrix) -> Self {
        let mut out = self.clone();
        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
                out[i][j] *= rhs[i][j];
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
        out[row] = out[row].iter().map(|entry| entry * c).collect();
        Ok(out)
    }

    pub fn rowadd(&self, r1: usize, r2: usize, c: f64) -> Result<Matrix, String> {
        if r1 >= self.nrows() && r2 >= self.nrows() {
            return Err(format!("Row index exceeds last row"));
        }
        let mut out = self.clone();
        for i in 0..self.ncols() {
            out[r1][i] += c * out[r2][i];
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

    pub fn trace(&self) -> Result<f64, String> {
        if !self.is_square() {
            return Err("Cannot compute trace of non-square matrix".to_owned());
        }
        let trace = self.rows.iter().enumerate().map(|(i, row)| row[i]).sum();
        Ok(trace)
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
                    if out[i][pcol].abs() > out[max_leading][pcol].abs() {
                        max_leading = i;
                    }
                }
            }

            if leading_values.is_empty() {
                pcol += 1;
                continue;
            }
            if prow != max_leading {
                out = out.rowswap(prow, max_leading).unwrap();
                swaps += 1
            }

            for i in (prow + 1)..rows {
                let mult = out[i][pcol] / out[prow][pcol];
                out = out.rowadd(i, prow, -mult).unwrap();
                out[i][pcol] = 0.0;
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
        let mut pcol = 0;

        for row in 0..rows {
            while out[row][pcol] == 0.0 {
                pcol += 1;
                if pcol >= cols {
                    return out;
                }
            }

            out = out.rowscale(row, 1.0 / out[row][pcol]).unwrap();
            out[row][pcol] = 1.0;

            for i in 0..row {
                out = out.rowadd(i, row, -out[i][pcol]).unwrap();
                out[i][pcol] = 0.0;
            }
        }
        out
    }

    pub fn inverse(&self) -> Result<Matrix, String> {
        if !self.is_invertible() {
            return Err("Matrix is not invertible".to_owned());
        }
        let augmented = self.augment_cols(Matrix::id(self.nrows())).unwrap();
        let reduced = augmented.RREF();
        let mut inverse_rows = Vec::new();
        for row in &reduced.rows {
            inverse_rows.push(row[self.ncols()..].to_vec())
        }
        Ok(Matrix { rows: inverse_rows })
    }

    pub fn powi(&self, power: i64) -> Result<Matrix, String> {
        if !self.is_square() {
            return Err("Cannot take powers of non-square matrix".to_owned());
        }
        let mut mult = if power >= 0 {
            self.clone()
        } else {
            self.inverse()?
        };
        let mut res = Matrix::id(self.nrows());
        let abs_power = power.abs();
        let mut pow2 = 1;
        loop {
            if abs_power & pow2 != 0 {
                res = res * mult.clone();
            }
            pow2 <<= 1;
            if pow2 > abs_power {
                break;
            }

            mult = mult.clone() * mult.clone();
        }

        Ok(res)
    }
    pub fn exp(&self) -> Result<Matrix, String> {
        if !self.is_square() {
            return Err("Cannot exponentiate non-square matrix".to_owned());
        }
        let mut res = Matrix::id(self.nrows());
        let mut mult = self.clone();
        for k in 1..21 {
            res = res + mult.clone().scale(1.0 / factorial(k) as f64);
            mult = mult * self.clone();
        }
        Ok(res)
    }

    pub fn QR(&self) -> Result<(Matrix, Matrix), String> {
        if !self.is_square() {
            return Err("Matrix is not square".to_owned());
        }

        let dim = self.ncols();
        let mut in_mat = self.clone();
        let mut p_matrices: Vec<Matrix> = vec![];

        // Go through each column
        for j in 0..dim - 1 {
            let mut a1 = Vector {
                entries: vec![0.0; dim - j],
            };
            let mut b1 = Vector {
                entries: vec![0.0; dim - j],
            };
            b1[0] = 1.0;
            // Set values for a
            for i in j..dim {
                a1[i - j] = in_mat[i][j]
            }

            let a1_norm = a1.norm();
            let sgn = a1[0].signum().neg();

            let u = a1 - (b1.scale(a1_norm).scale(sgn));
            let n = u.normalised();
            let id = Matrix::id(dim - j);
            let p_temp = id - n.outer_mul(&n);

            let mut p = Matrix::id(dim);
            for row in j..dim {
                for col in j..dim {
                    p[row][col] = p_temp[row - j][col - j];
                }
            }
            in_mat = p.clone() * in_mat;
            p_matrices.push(p);
        }

        // Compute Q
        let length = p_matrices.len();
        let mut Q = p_matrices[0].clone();
        for i in 1..length {
            Q = Q * p_matrices[i].clone().transpose();
        }
        // Compute R
        let mut R = p_matrices[length - 1].clone();
        for i in (0..=length - 2).rev() {
            R = R * p_matrices[i].clone();
        }
        R = R.clone() * self.clone();

        Ok((Q, R))
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
