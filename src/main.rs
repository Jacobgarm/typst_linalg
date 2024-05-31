mod common;
mod convert;
mod matrix;
mod vector;

use matrix::*;
fn main() {
    let m = Matrix {
        rows: vec![
            vec![4.0, 5.0, 3.0],
            vec![0.0, 0.0, 6.0],
            vec![0.0, 7.0, 0.0],
        ],
    };
    let a = m.inverse();
    //let s = m.det();
    dbg!(a);
}
