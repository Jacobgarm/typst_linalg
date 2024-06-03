use core::fmt;

use crate::common::*;
use crate::convert::Convertable;
use crate::vector::Vector;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>
where
    T: Scalar,
{
    pub rows: Vec<Vec<T>>,
}

pub trait Echelon: Sized {
    fn echelon(&self) -> (Self, usize);
}

impl<T: Scalar> std::fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = "".to_owned();
        for i in 0..self.nrows() {
            res.push('[');
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

impl<T: Scalar> std::ops::Add for Matrix<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.ncols(), rhs.ncols());
        assert_eq!(self.nrows(), rhs.nrows());
        let mut out = self;
        for i in 0..out.nrows() {
            for j in 0..out.ncols() {
                out[i][j] += rhs[i][j];
            }
        }
        out
    }
}

impl<T: Scalar> std::ops::Add for &Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.ncols(), rhs.ncols());
        assert_eq!(self.nrows(), rhs.nrows());
        let mut out = self.clone();
        for i in 0..out.nrows() {
            for j in 0..out.ncols() {
                out[i][j] += rhs[i][j];
            }
        }
        out
    }
}

impl<T: Scalar> std::ops::Neg for Matrix<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut out = self;
        for i in 0..out.nrows() {
            for j in 0..out.ncols() {
                out[i][j] = -out[i][j];
            }
        }
        out
    }
}

impl<T: Scalar> std::ops::Neg for &Matrix<T> {
    type Output = Matrix<T>;
    fn neg(self) -> Self::Output {
        let mut out = self.clone();
        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
                out[i][j] *= -T::one();
            }
        }
        out
    }
}

impl<T: Scalar> std::ops::Sub for Matrix<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = self;
        for i in 0..out.nrows() {
            for j in 0..out.ncols() {
                out[i][j] -= rhs[i][j];
            }
        }
        out
    }
}

impl<T: Scalar> std::ops::Sub for &Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = self.clone();
        for i in 0..out.nrows() {
            for j in 0..out.ncols() {
                out[i][j] -= rhs[i][j];
            }
        }
        out
    }
}

impl<T: Scalar> std::ops::Mul for Matrix<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
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

impl<T: Scalar> std::ops::Mul for &Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: Self) -> Self::Output {
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

impl<T: Scalar> std::ops::Index<usize> for Matrix<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl<T: Scalar> std::ops::IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}

impl<T: Scalar> Matrix<T> {
    fn nrows(&self) -> usize {
        self.rows.len()
    }

    fn ncols(&self) -> usize {
        self.rows[0].len()
    }

    fn is_square(&self) -> bool {
        self.nrows() == self.ncols()
    }

    pub fn filled(rows: usize, cols: usize, value: T) -> Self {
        let mut out: Vec<Vec<T>> = Vec::new();
        let filled_row = vec![value; cols];
        for _ in 0..rows {
            out.push(filled_row.clone());
        }
        Matrix { rows: out }
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        Matrix::filled(rows, cols, T::zero())
    }

    pub fn id(dim: usize) -> Self {
        let mut out = Matrix::zero(dim, dim);
        for i in 0..dim {
            out[i][i] = T::one();
        }
        out
    }

    fn get_vector(&self, col: usize) -> Vector<T> {
        let rows = self.nrows();
        let mut out = Vector::zero(rows);
        for i in 0..rows {
            out[i] = self[i][col];
        }
        out
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

    fn scale(&self, scalar: T) -> Self {
        let mut out = self.clone();
        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
                out[i][j] *= scalar;
            }
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
                if i != j && !self[i][j].is_zero() {
                    return false;
                }
            }
        }
        true
    }

    pub fn mul_vector(&self, v: &Vector<T>) -> Result<Vector<T>, String> {
        if v.dim() != self.ncols() {
            return Err("Vector does not have same dimension as matrix".to_owned());
        }
        let mut res = Vec::new();
        for i in 0..self.nrows() {
            let mut entry = T::zero();
            for j in 0..self.ncols() {
                entry += self[i][j] * v[j];
            }
            res.push(entry);
        }
        Ok(Vector { entries: res })
    }

    pub fn rowswap(&self, r1: usize, r2: usize) -> Result<Self, String> {
        if r1 >= self.nrows() || r2 >= self.nrows() {
            return Err("Row index exceeds last row".to_owned());
        }
        let mut out = self.clone();
        out.rows.swap(r1, r2);
        Ok(out)
    }

    pub fn rowscale(&self, row: usize, c: T) -> Result<Self, String> {
        if row >= self.nrows() {
            return Err("Row index exceeds last row".to_owned());
        }
        let mut out = self.clone();
        out[row] = out[row].iter().map(|entry| *entry * c).collect();
        Ok(out)
    }

    pub fn rowadd(&self, r1: usize, r2: usize, c: T) -> Result<Self, String> {
        if r1 >= self.nrows() && r2 >= self.nrows() {
            return Err("Row index exceeds last row".to_owned());
        }
        if r1 == r2 {
            return Err("Cannot add a row to itself".to_owned());
        }
        let mut out = self.clone();
        for (i, entry) in self[r2].iter().enumerate() {
            out[r1][i] += c * *entry;
        }
        Ok(out)
    }

    fn hadamard(&self, rhs: &Self) -> Self {
        let mut out = self.clone();
        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
                out[i][j] *= rhs[i][j];
            }
        }
        out
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Result<Self, String> {
        if row >= self.nrows() {
            return Err("Cannot remove row that does not exist".to_owned());
        }
        if col >= self.ncols() {
            return Err("Cannot remove column that does not exist".to_owned());
        }
        let mut out = Matrix { rows: vec![] };
        for i in 0..self.nrows() {
            if i == row {
                continue;
            }
            let mut mat_row = self.rows[i].clone();
            mat_row.remove(col);
            out.rows.push(mat_row)
        }
        Ok(out)
    }

    pub fn embed_matrix(&self, other: &Self, row: usize, col: usize) -> Self {
        let mut out = self.clone();
        for i in 0..other.nrows() {
            for j in 0..other.ncols() {
                out[i + row][j + col] = other[i][j];
            }
        }
        out
    }

    fn augment_cols(&self, right: &Self) -> Result<Self, String> {
        if self.nrows() != right.nrows() {
            return Err("Cannot horizontally augment matrices of different heights".to_owned());
        }
        let mut augmented = self.clone();
        for i in 0..self.nrows() {
            augmented[i].extend(right[i].clone());
        }
        Ok(augmented)
    }

    fn augment_rows(&self, below: &Self) -> Result<Self, String> {
        if self.ncols() != below.ncols() {
            return Err("Cannot vertically augment matrices of different widths".to_owned());
        }
        let mut augmented = self.clone();
        for i in 0..below.nrows() {
            augmented.rows.push(below[i].clone());
        }
        Ok(augmented)
    }

    pub fn trace(&self) -> Result<T, String> {
        if !self.is_square() {
            return Err("Cannot compute trace of non-square matrix".to_owned());
        }
        let trace = self.rows.iter().enumerate().map(|(i, row)| row[i]).sum();
        Ok(trace)
    }

    fn is_invertible(&self) -> bool {
        self.is_square() && !self.echelon().0[self.nrows() - 1][self.ncols() - 1].is_zero()
    }

    pub fn det(&self) -> Result<T, String> {
        if !self.is_square() {
            return Err("Non-square matrix has no determinant".to_owned());
        }
        let (mat_ref, swaps) = self.echelon();
        let mut determinant = if swaps % 2 == 0 { T::one() } else { -T::one() };
        for i in 0..self.ncols() {
            determinant *= mat_ref[i][i];
        }
        Ok(determinant)
    }

    pub fn reduced_echelon(&self) -> Self {
        let (mut out, _) = self.echelon();
        let rows = self.nrows();
        let cols = self.ncols();
        let mut pcol = 0;

        for row in 0..rows {
            while out[row][pcol].is_zero() {
                pcol += 1;
                if pcol >= cols {
                    return out;
                }
            }

            out = out.rowscale(row, T::one() / out[row][pcol]).unwrap();
            out[row][pcol] = T::one();

            for i in 0..row {
                out = out.rowadd(i, row, -out[i][pcol]).unwrap();
                out[i][pcol] = T::zero();
            }
        }
        out
    }

    pub fn inverse(&self) -> Result<Self, String> {
        if !self.is_invertible() {
            return Err("Matrix is not invertible".to_owned());
        }
        let augmented = self.augment_cols(&Matrix::id(self.nrows())).unwrap();
        let reduced = augmented.reduced_echelon();
        let mut inverse_rows = Vec::new();
        for row in &reduced.rows {
            inverse_rows.push(row[self.ncols()..].to_vec())
        }
        Ok(Matrix { rows: inverse_rows })
    }

    pub fn powi(&self, power: i64) -> Result<Self, String> {
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
}

impl<T: Scalar> Echelon for Matrix<T> {
    default fn echelon(&self) -> (Self, usize) {
        let mut out = self.clone();
        let rows = self.nrows();
        let cols = self.ncols();
        let mut prow = 0;
        let mut swaps = 0;

        for pcol in 0..cols {
            if out[prow][pcol].is_zero() {
                let mut pivot = false;
                for i in prow + 1..rows {
                    if !out[i][pcol].is_zero() {
                        out = out.rowswap(prow, i).unwrap();
                        swaps += 1;
                        prow = i;
                        pivot = true;
                        break;
                    }
                }
                if !pivot {
                    continue;
                }
            }
            if prow == rows - 1 {
                break;
            }

            for i in prow + 1..rows {
                out = out
                    .rowadd(i, prow, -out[i][pcol] / out[prow][pcol])
                    .unwrap();
                out[i][pcol] = T::zero();
            }
            prow += 1;
            if prow == rows - 1 {
                break;
            }
        }
        (out, swaps)
    }
}

impl Echelon for Matrix<f64> {
    fn echelon(&self) -> (Self, usize) {
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
}

impl<T: Scalar + From<f64>> Matrix<T> {
    pub fn exp(&self) -> Result<Self, String> {
        if !self.is_square() {
            return Err("Cannot exponentiate non-square matrix".to_owned());
        }
        let mut res = Matrix::id(self.nrows());
        let mut mult = self.clone();
        for k in 1..21 {
            res = res + mult.clone().scale((1.0 / factorial(k) as f64).into());
            mult = mult * self.clone();
        }
        Ok(res)
    }
}

impl Matrix<f64> {
    fn givens_rotation(dim: usize, i: usize, j: usize, angle: f64) -> Self {
        let mut out = Matrix::id(dim);
        let c = angle.cos();
        let s = angle.sin();
        out[i][i] = c;
        out[j][j] = c;
        out[i][j] = s;
        out[j][i] = -s;
        out
    }

    pub fn rotation_2d(angle: f64) -> Self {
        Matrix::givens_rotation(2, 1, 0, angle)
    }

    pub fn rotation_x_3d(angle: f64) -> Self {
        Matrix::givens_rotation(3, 2, 1, angle)
    }

    pub fn rotation_y_3d(angle: f64) -> Self {
        Matrix::givens_rotation(3, 2, 0, angle)
    }

    pub fn rotation_z_3d(angle: f64) -> Self {
        Matrix::givens_rotation(3, 1, 0, angle)
    }

    pub fn householder_standard(v: Vector<f64>) -> Self {
        let dim = v.dim();
        let mut e1 = Vector {
            entries: vec![0.0; dim],
        };
        e1[0] = 1.0;
        let sgn = v[0].signum();
        let u = v.clone() + e1.scale(sgn).scale(v.norm());
        let n = u.normalised();
        Matrix::id(dim) - n.outer_mul(&n).scale(2.0)
    }

    pub fn qr_decomposition(&self) -> Result<(Self, Self), String> {
        let big = self.ncols() >= 8 && self.nrows() >= 8;
        let cols = self.ncols();
        let mut m = self.clone();
        let mut p_matrices: Vec<Matrix<f64>> = vec![];
        let dim = self.nrows();

        for i in 0..cols {
            if i > 0 {
                m = m.submatrix(0, 0).unwrap();
            }
            let v = m.get_vector(0);
            let p = Matrix::householder_standard(v);

            // Fix for now, since something happens that shouldn't happen if not. Very slight undershoot (around 15th-16th decimal), but it generally works.
            if p.ncols() <= 2 && big {
                break;
            }
            // println!("Current p matrix is:\n{}", p);
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
            // println!("Q is at step {}\n{}", i, q);
        }
        let mut r = p_matrices[num_matrices - 1].clone();
        for i in (0..num_matrices - 1).rev() {
            r = r * p_matrices[i].clone();
        }
        r = r * self.clone();
        Ok((q, r))
    }
}

impl<T: Scalar> Convertable for Matrix<T> {
    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let s = std::str::from_utf8(bytes).unwrap().to_owned();

        let mut rows: Vec<Vec<T>> = Vec::new();
        let mut row_length = usize::MAX;
        for row_str in s.split(';') {
            let mut row = Vec::new();
            for entry in row_str.split(',') {
                let res_float = T::from_str(entry);
                match res_float {
                    Ok(float) => row.push(float),
                    Err(_) => return Err("Unable to parse number in matrix".to_owned()),
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
