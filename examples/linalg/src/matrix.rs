use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    rows: Vec<Vec<f64>>
}

impl std::ops::Add for Matrix {
    type Output = Self;
    fn add(self, rhs: Matrix) -> Self::Output {
        let mut out = self.clone();
        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
               out.rows[i][j] += rhs.rows[i][j];
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
                out.rows[i][j] -= rhs.rows[i][j];
            }
        }
        out
    }
}

impl std::ops::Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut out = Matrix::zero(self.nrows(), self.ncols());
        for i in 0..s
    }
}
    
impl Matrix {

    fn zero(rows: usize, cols: usize) -> Self {
        let mut out: Vec<Vec<f64>> = Vec::new();
        let zero_vec = vec![0 as f64 ; cols];
        for _ in 0..rows {
            out.push(zero_vec.clone());
        }       
        Matrix { rows: out }
    }

    fn nrows(&self) -> usize {
        self.rows.len()
    }

    fn ncols(&self) -> usize {
        self.rows[0].len()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let s = std::str::from_utf8(bytes).unwrap().to_owned();
        let mut rows = Vec::new();
        for row in s.split(';') {
            rows.push(
                row.split(',').map(|entry| f64::from_str(entry).unwrap()).collect()
            )
        }
        Matrix {
            rows
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let s = self.rows.iter().map(
            |row| row.iter().map(
                |entry| entry.to_string()).collect::<Vec<String>>().join(",")
            ).collect::<Vec<String>>().join(";");
        s.as_bytes().to_vec()
    }
}

