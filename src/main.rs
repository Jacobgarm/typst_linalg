mod common;
mod convert;
mod matrix;
mod vector;

use matrix::*;
use vector::*;
fn main() {
    let m = Matrix {
        rows: vec![
            vec![2.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0],
            vec![0.0, 0.0, 2.0],
        ],
    };
    // let a = m.inverse();
    //let s = m.det();
    let v = Vector {
        entries: vec![1.0, 2.0, 3.0],
    };
    dbg!(v.outer_mul(&v));
}
