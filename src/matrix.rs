use core::fmt;
use std::str::FromStr;

use crate::common::*;
use crate::convert::Convertable;
use crate::vector::Vector;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows: Vec<Vec<f64>>,
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = "".to_owned();
        for i in 0..self.nrows() {
            res.push_str("[");
            for j in 0..self.ncols() {
                res.push_str(self[i][j].clone().to_string().as_str());
                if j == self.ncols() - 1 {
                    continue;
                }
                res.push_str(", ")
            }
            res.push_str("]\n");
        }
        write!(f, "{}", res)
    }
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
        assert_eq!(self.ncols(), rhs.nrows());
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

    pub fn id(dim: usize) -> Self {
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

    fn get_vector(&self, col: usize) -> Vector {
        let rows = self.nrows();
        let mut out = Vector {
            entries: vec![0.0; rows],
        };
        for i in 0..rows {
            out[i] = self[i][col].clone();
        }
        out
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

    pub fn mul_vector(&self, v: Vector) -> Result<Vector, String> {
        if v.dim() != self.ncols() {
            return Err("Vector does nor have same dimension as matrix".to_owned());
        }
        let mut res = Vec::new();
        for i in 0..self.nrows() {
            let mut entry = 0.0;
            for j in 0..self.ncols() {
                entry += self[i][j] * v[j];
            }
            res.push(entry);
        }
        Ok(Vector { entries: res })
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

    pub fn submatrix(&self, row: usize, col: usize) -> Result<Matrix, String> {
        let rows = self.nrows();
        let cols = self.ncols();
        let mut out = Matrix { rows: vec![] };
        if row >= self.nrows() {
            return Err(format!("Cannot remove row that does not exist"));
        }
        if col >= self.ncols() {
            return Err(format!("Cannot remove column that does not exist"));
        }
        for i in 0..rows {
            if i == row {
                continue;
            }
            let mut mat_row = self.rows[i].clone();
            mat_row.remove(col);
            out.rows.push(mat_row)
        }
        Ok(out)
    }

    pub fn embed_matrix(&self, other: &Self, row: usize, col: usize) -> Matrix {
        let mut out = self.clone();
        for i in 0..other.nrows() {
            for j in 0..other.ncols() {
                out[i + row][j + col] = other[i][j].clone();
            }
        }
        out
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

    pub fn householder_standard(v: Vector) -> Matrix {
        let dim = v.dim();
        let mut e1 = Vector {
            entries: vec![0.0; dim],
        };
        e1[0] = 1.0;
        let sgn = v[0].signum();
        let u = v.clone() + e1.scale(sgn).scale(v.norm());
        let n = u.normalised();
        let beta = 2.0 / (n.inner(&n));
        Matrix::id(dim) - n.outer_mul(&n).scale(beta)
    }

    pub fn QR(&self) -> Result<(Matrix, Matrix), String> {
        let cols = self.ncols();
        let mut m = self.clone();
        let mut p_matrices: Vec<Matrix> = vec![];
        let dim = self.nrows();

        for i in 0..cols {
            if i > 0 {
                m = m.submatrix(0, 0).unwrap();
            }
            let v = m.get_vector(0);
            let v_clone = v.clone();
            let p = Matrix::householder_standard(v);

            let embedded_p = Matrix::id(dim).embed_matrix(&p.clone(), i, i);
            p_matrices.push(embedded_p.clone());

            if i == cols - 1 {
                continue;
            }
            m = p * m;
        }
        let num_matrices = p_matrices.len();
        let mut q = p_matrices[0].clone();
        for i in 1..num_matrices {
            q = q * p_matrices[i].clone();
            println!("Q is at step {}\n{}", i, q);
        }
        let mut r = p_matrices[num_matrices - 1].clone();
        for i in (0..num_matrices - 1).rev() {
            r = r * p_matrices[i].clone();
        }
        r = r * self.clone();
        Ok((q, r))
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
