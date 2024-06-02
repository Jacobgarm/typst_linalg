#![allow(dead_code)]
#![allow(non_snake_case)]
#![feature(specialization)]
use fraction::Fraction;

mod common;
mod convert;
mod matrix;
mod vector;

use matrix::*;
use num::Zero;
use vector::*;

#[allow(unused)]
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
    let mut I: Matrix<f64> = Matrix::id(15);
    let A = Matrix::filled(12, 14, 1.0);
    let v = Vector {
        entries: vec![2.0; 4],
    };
    // println!("{}", I.embed_matrix(&m, 1, 1));
    // let a = m.inverse();
    //let s = m.det();
    let (Q, R) = m.QR().unwrap();
    println!("Q:\n{},\nR:\n{}", Q, R);
    println!("Product is:\n{}", Q * R);
    println!("Rotation:\n{}", Matrix::rotation_y_3d(2.0));
    let fv1 = Vector {
        entries: vec![Fraction::new(1u64, 2u64); 3],
    };
    let fv2 = Vector {
        entries: vec![Fraction::new(3u64, 4u64); 3],
    };
    println!("Inner {}", fv1.inner(&fv2));
    let mut A = Matrix::filled(2, 2, Fraction::from(0.25));
    A[0][0] = Fraction::zero();
    let mut B = Matrix::filled(2, 2, 0.25_f64);
    B[0][0] = 0.1;
    println!("{}", B.REF().0);
}
