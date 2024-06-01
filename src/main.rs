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
    let M = Matrix {
        rows: vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0]
        ]
    };
    let I = Matrix::id(5);
    let v = Vector {
        entries : vec![1.0; 4]
    };
    // println!("{}", I.embed_matrix(&m, 1, 1));
    // let a = m.inverse();
    //let s = m.det();
    let (Q, R) = M.QR().unwrap();
    println!("Q:\n{},\nR:\n{}", Q , R);
    println!("Product is:\n{}", Q * R);
}
