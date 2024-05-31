mod common;
mod convert;
mod matrix;
mod vector;

use matrix::*;
fn main() {
    let m = Matrix {
        rows: vec![
            vec![2.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0],
            vec![0.0, 0.0, 2.0],
        ],
    };
    let a = m.inverse();
    //let s = m.det();
    dbg!(m.powi(3));
}
