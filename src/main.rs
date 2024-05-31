mod common;
mod convert;
mod matrix;
mod vector;

use matrix::*;
use vector::*;
fn main() {
    let m = Matrix {
        rows: vec![
            vec![0.5, 0.75, 0.5],
            vec![1.0, 0.5, 0.75],
            vec![0.25, 0.25, 0.25],
        ],
    };
    // let a = m.inverse();
    //let s = m.det();
    let (Q, R) = m.QR().unwrap();
    dbg!(Q, R);
}
