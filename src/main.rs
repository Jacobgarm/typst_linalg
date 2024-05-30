mod matrix;
use matrix::*;
use std::str::FromStr;
fn main() {
    let m = Matrix {
        rows: vec![
            vec![1.0, -4.0, 2.0],
            vec![0.0, 0.0, 5.0],
            vec![0.0, 0.0, 0.0],
        ],
    };
    let (a, _) = m.REF();
    let s = m.det();
    dbg!(a);
}

