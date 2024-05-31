use std::str::FromStr;

use crate::common::*;
use crate::convert::Convertable;

#[derive(Debug, Clone, PartialEq)]
struct Vector {
    entries: Vec<f64>,
}

impl std::ops::Add for Vector {
    type Output = Self;
    fn add(self, rhs: Vector) -> Self::Output {
        let mut out = self.clone();
        for i in 0..self.dim() {
            out[i] += rhs[i];
        }
        out
    }
}

impl std::ops::Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl std::ops::IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl Vector {
    pub fn dim(&self) -> usize {
        self.entries.len()
    }

    pub fn length(&self) -> f64 {
        self.entries.iter().map(|x| x * x).sum::<f64>().sqrt()
    }
}

impl Convertable for Vector {
    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let s = std::str::from_utf8(bytes).unwrap().to_owned();

        let mut entries: Vec<f64> = Vec::new();
        for entry in s.split(',') {
            let res_float = f64::from_str(entry);
            match res_float {
                Ok(float) => entries.push(float),
                Err(err) => return Err(err.to_string()),
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
