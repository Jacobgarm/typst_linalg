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
        let mut out = self.clone();
        for i in 0..self.dim() {
            out[i] += rhs[i];
        }
        out
    }
}

impl<T: Scalar> std::ops::Sub for Vector<T> {
    type Output = Self;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        let mut out = self.clone();
        for i in 0..self.dim() {
            out[i] -= rhs[i];
        }
        out
    }
}

impl<T: Scalar> std::ops::Neg for Vector<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut out = self.clone();
        for i in 0..self.dim() {
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

impl<T: Scalar> Vector<T> {
    pub fn dim(&self) -> usize {
        self.entries.len()
    }

    pub fn scale(&self, c: T) -> Vector<T> {
        Vector {
            entries: self.entries.iter().map(|x| *x * c).collect(),
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

    pub fn outer_mul(&self, other: &Self) -> Matrix {
        let self_mat = Matrix {
            rows: vec![self.entries.clone()],
        }
        .transpose();
        let other_mat = Matrix {
            rows: vec![other.entries.clone()],
        };
        self_mat * other_mat
    }

    pub fn inner(&self, other: &Self) -> f64 {
        zip(self.entries.iter(), other.entries.iter())
            .map(|(a, b)| a * b)
            .sum::<f64>()
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

    pub fn inner(&self, other: &Self) -> Complex64 {
        zip(self.entries.iter(), other.entries.iter())
            .map(|(a, b)| a * b.conj())
            .sum::<Complex64>()
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
