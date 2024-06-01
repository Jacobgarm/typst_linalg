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
            vec![1.0, -1.0, 0.0],
        ],
    };
    let M = Matrix {
        rows: vec![
            vec![10.0, 11.0, 12.0],
            vec![10.0, 11.0, 12.0],
            vec![10.0, 11.0, 12.0],
            vec![10.0, 11.0, 12.0],
            vec![10.0, 11.0, 12.0],
            vec![10.0, 11.0, 12.0],
            vec![10.0, 11.0, 12.0],
            vec![10.0, 11.0, 12.0],
            vec![10.0, 11.0, 12.0],
            vec![10.0, 11.0, 12.0],
            vec![10.0, 11.0, 12.0],
        ],
    };
    let mut I = Matrix::id(15);
    let A = Matrix::filled(12, 14, 1.0);
    let v = Vector {
        entries: vec![1.0; 4],
    };
    // println!("{}", I.embed_matrix(&m, 1, 1));
    // let a = m.inverse();
    //let s = m.det();
    let (Q, R) = m.QR().unwrap();
    println!("Q:\n{},\nR:\n{}", Q, R);
    println!("Product is:\n{}", Q * R);
    println!("Rotation:\n{}", Matrix::rotation_y_3d(3.141592));
}
