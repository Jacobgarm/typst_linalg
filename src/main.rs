mod common;
mod convert;
mod matrix;
mod vector;

use matrix::*;
use vector::*;
fn main() {
    let m = Matrix {
        rows: vec![
           vec![1.0, -1.0, 4.0],
           vec![1.0, 4.0, -2.0],
           vec![1.0, 4.0, 2.0],
           vec![1.0, -1.0, 0.0]
        ],
    };
    let v = Vector {
        entries : vec![1.0; 4]
    };
    // let a = m.inverse();
    //let s = m.det();
    let (Q, R) = m.QR().unwrap();
    dbg!(Q , R);
}
