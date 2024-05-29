use std::str::FromStr;

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
                out.rows[i][j] += rhs.rows[i][j];
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
                out.rows[i][j] *= -1.0;
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
        let mut out = Matrix::zero(self.nrows(), rhs.ncols());
        for i in 0..self.nrows() {
            for j in 0..rhs.ncols() {
                for k in 0..rhs.nrows() {
                    out.rows[i][j] += self.rows[i][k] * rhs.rows[k][j];
                }
            }
        }
        out
    }
}

impl Matrix {
    fn zero(rows: usize, cols: usize) -> Self {
        let mut out: Vec<Vec<f64>> = Vec::new();
        let zero_vec = vec![0 as f64; cols];
        for _ in 0..rows {
            out.push(zero_vec.clone());
        }
        Matrix { rows: out }
    }

    fn id(dim: usize) -> Self {
        let mut out = Matrix::zero(dim, dim);
        for i in 0..dim {
            out.rows[i][i] = 1.0;
        }
        out
    }

    fn nrows(&self) -> usize {
        self.rows.len()
    }

    fn ncols(&self) -> usize {
        self.rows[0].len()
    }

    pub fn rowswap(&self, r1: usize, r2: usize) -> Matrix {
        let mut out = self.clone();
        out.rows.swap(r1, r2);
        out
    }

    pub fn rowscale(&self, row: usize, c: f64) -> Matrix {
        let mut out = self.clone();
        out.rows[row] = out.rows[row].iter().map(|entry| entry * c).collect();
        out
    }

    pub fn rowadd(&self, r1: usize, r2: usize, c: f64) -> Matrix {
        let mut out = self.clone();
        for i in 0..self.ncols() {
            out.rows[r1][i] += c * out.rows[r2][i];
        }
        out
    }

    pub fn REF(&self) -> (Matrix, usize) {
        let mut out = self.clone();
        let rows = self.nrows();
        let cols = self.ncols();
        let mut prow = 0;
        let mut pcol = 0;
        let mut swaps = 0;

        while (prow < rows && pcol < cols) {
            let mut leading_values = vec![0; rows - prow];
            let mut max_leading = prow;

            for i in prow..rows {
                if out.rows[i][pcol] != 0.0 {
                    leading_values[i - prow] = i;
                    if out.rows[i][pcol].abs() > out.rows[max_leading][pcol].abs() {
                        max_leading = i;
                    }
                }
            }

            if leading_values.len() == 0 {
                pcol += 1;
                continue;
            }

            if prow != max_leading {
                out = out.rowswap(prow, max_leading);
                swaps += 1
            }

            for i in (prow + 1)..rows {
                let mult = out.rows[i][pcol] / out.rows[prow][pcol];
                out = out.rowadd(i, prow, -mult);
                out.rows[i][pcol] = 0.0;
            }

            prow += 1;
            pcol += 1;
        }
        (out, swaps)
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let s = std::str::from_utf8(bytes).unwrap().to_owned();
        let mut rows = Vec::new();
        for row in s.split(';') {
            rows.push(
                row.split(',')
                    .map(|entry| f64::from_str(entry).unwrap())
                    .collect(),
            )
        }
        Matrix { rows }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
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

fn truncate_zeroes(num_str: String) -> String {
    let mut sep_found = false;
    let mut zeroes = 0;
    for (i, c) in num_str.chars().enumerate() {
        if c == '.' {
            sep_found = true;
            continue;
        }
        if sep_found && c == '0' {
            zeroes += 1;
            if zeroes == 10 {
                return (num_str[..=i]).to_owned();
            }
        } else {
            zeroes = 0;
        }
    }
    num_str
}
