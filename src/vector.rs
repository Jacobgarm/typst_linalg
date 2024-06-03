use fraction::Fraction;
use num::complex::Complex64;
use std::iter::zip;
use std::vec;

use crate::convert::Convertable;
use crate::{common::*, Matrix};

#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T>
where
    T: Scalar,
{
    pub entries: Vec<T>,
}

impl<T: Scalar> std::ops::Add for Vector<T> {
    type Output = Self;
    fn add(self, rhs: Vector<T>) -> Self::Output {
        assert_eq!(self.dim(), rhs.dim());
        let mut out = self;
        for i in 0..out.dim() {
            out[i] += rhs[i];
        }
        out
    }
}

impl<T: Scalar> std::ops::Add for &Vector<T> {
    type Output = Vector<T>;
    fn add(self, rhs: &Vector<T>) -> Self::Output {
        assert_eq!(self.dim(), rhs.dim());
        let mut out = self.clone();
        for i in 0..out.dim() {
            out[i] += rhs[i];
        }
        out
    }
}

impl<T: Scalar> std::ops::Sub for Vector<T> {
    type Output = Self;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        assert_eq!(self.dim(), rhs.dim());
        let mut out = self;
        for i in 0..out.dim() {
            out[i] -= rhs[i];
        }
        out
    }
}

impl<T: Scalar> std::ops::Sub for &Vector<T> {
    type Output = Vector<T>;
    fn sub(self, rhs: &Vector<T>) -> Self::Output {
        assert_eq!(self.dim(), rhs.dim());
        let mut out = self.clone();
        for i in 0..out.dim() {
            out[i] -= rhs[i];
        }
        out
    }
}

impl<T: Scalar> std::ops::Neg for Vector<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut out = self;
        for i in 0..out.dim() {
            out[i] = -out[i];
        }
        out
    }
}

impl<T: Scalar> std::ops::Neg for &Vector<T> {
    type Output = Vector<T>;
    fn neg(self) -> Self::Output {
        let mut out = self.clone();
        for i in 0..out.dim() {
            out[i] = -out[i];
        }
        out
    }
}

impl<T: Scalar> std::ops::Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl<T: Scalar> std::ops::IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl<T: Scalar> From<Vec<T>> for Vector<T> {
    fn from(value: Vec<T>) -> Self {
        Vector { entries: value }
    }
}

impl<T: Scalar> From<Vector<T>> for Vec<T> {
    fn from(value: Vector<T>) -> Self {
        value.entries
    }
}

impl<T: Scalar> Vector<T> {
    pub fn dim(&self) -> usize {
        self.entries.len()
    }

    pub fn zero(dim: usize) -> Self {
        Vector::from(vec![T::zero(); dim])
    }

    pub fn standard_basis(dim: usize, i: usize) -> Self {
        let mut e = Self::zero(dim);
        e[i] = T::one();
        e
    }

    pub fn scale(&self, c: T) -> Vector<T> {
        Vector::from(self.entries.iter().map(|x| *x * c).collect::<Vec<T>>())
    }

    pub fn row_matrix(&self) -> Matrix<T> {
        Matrix {
            rows: vec![self.entries.clone()],
        }
    }

    pub fn column_matrix(&self) -> Matrix<T> {
        self.row_matrix().transpose()
    }

    pub fn outer_mul(&self, other: &Self) -> Matrix<T> {
        let self_mat = self.column_matrix();
        let other_mat = other.row_matrix();
        self_mat * other_mat
    }

    pub fn cross_product(&self, rhs: &Self) -> Result<Self, String> {
        match (self.dim(), rhs.dim()) {
            (0, 0) => Ok(Vector::zero(0)),
            (1, 1) => Ok(Vector::zero(1)),
            (3, 3) => Ok(Vector::from(vec![
                self[1] * rhs[2] - self[2] * rhs[1],
                self[2] * rhs[0] - self[0] * rhs[2],
                self[0] * rhs[1] - self[1] * rhs[0],
            ])),
            (7, 7) => Ok(Vector::from({
                let mut prod = vec![];
                for i in 0..7 {
                    prod.push(
                        self[(i + 1) % 7] * rhs[(i + 3) % 7] - self[(i + 3) % 7] * rhs[(i + 1) % 7]
                            + self[(i + 2) % 7] * rhs[(i + 6) % 7]
                            - self[(i + 6) % 7] * rhs[(i + 2) % 7]
                            + self[(i + 4) % 7] * rhs[(i + 5) % 7]
                            - self[(i + 5) % 7] * rhs[(i + 4) % 7],
                    )
                }
                prod
            })),
            _ => Err(
                "Cross product is only defined for pairs of 0, 1, 3 or 7 dimensional vectors"
                    .to_owned(),
            ),
        }
    }
}

impl Vector<f64> {
    pub fn norm(&self) -> f64 {
        self.entries.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    pub fn normalised(&self) -> Vector<f64> {
        self.scale(1.0 / self.norm())
    }

    pub fn inner(&self, other: &Self) -> f64 {
        zip(self.entries.iter(), other.entries.iter())
            .map(|(a, b)| a * b)
            .sum::<f64>()
    }
}

impl Vector<Fraction> {
    pub fn inner(&self, other: &Self) -> Fraction {
        zip(self.entries.iter(), other.entries.iter())
            .map(|(a, b)| a * b)
            .sum::<Fraction>()
    }
}

impl Vector<Complex64> {
    pub fn norm(&self) -> f64 {
        self.entries
            .iter()
            .map(|x| (x * x.conj()).re)
            .sum::<f64>()
            .sqrt()
    }

    pub fn normalised(&self) -> Self {
        self.scale((1.0 / self.norm()).into())
    }

    pub fn inner(&self, other: &Self) -> Complex64 {
        zip(self.entries.iter(), other.entries.iter())
            .map(|(a, b)| a * b.conj())
            .sum::<Complex64>()
    }

    pub fn adjoint_mul(&self, other: &Self) -> Matrix<Complex64> {
        let self_mat = self.column_matrix();
        let other_mat = other.column_matrix().adjoint();
        self_mat * other_mat
    }
}

impl<T: Scalar> Convertable for Vector<T> {
    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let s = std::str::from_utf8(bytes).unwrap().to_owned();

        let mut entries: Vec<T> = Vec::new();
        for entry in s.split(',') {
            let res = T::from_str(entry);
            match res {
                Ok(num) => entries.push(num),
                Err(_) => return Err("Unable to parse number in vector".to_owned()),
            }
        }
        Ok(Vector { entries })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let s = self
            .entries
            .iter()
            .map(|entry| {
                let entry_s = entry.to_string();
                truncate_zeroes(entry_s)
            })
            .collect::<Vec<String>>()
            .join(",");
        s.as_bytes().to_vec()
    }
}
