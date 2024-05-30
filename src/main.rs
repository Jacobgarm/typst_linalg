mod matrix;
use matrix::*;
use std::str::FromStr;
fn main() {
    let m = Matrix {
        rows: vec![
            vec![1.0, -4.0, 2.0],
            vec![0.0, 0.0, 5.0],
            vec![0.0, 2.0, 0.0],
        ],
    };
    let (s, _) = m.REF();
    print!("{:?}", s);
}

