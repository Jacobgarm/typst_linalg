#![allow(dead_code)]
#![feature(min_specialization)]
use fraction::Fraction;

mod common;
mod convert;
mod matrix;
mod vector;

use matrix::*;
use vector::*;
type F = Fraction;

#[allow(unused)]
fn main() {
    let m1 = Matrix {
        rows: vec![
            vec![1.0, -1.0, 4.0],
            vec![1.0, 4.0, -2.0],
            vec![1.0, 4.0, 2.0],
            vec![1.0, -1.0, 0.0],
        ],
    };
    let m2 = Matrix {
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
    let mut id: Matrix<f64> = Matrix::id(15);
    let a = Matrix::filled(12, 14, 1.0);
    let v = Vector {
        entries: vec![2.0; 4],
    };
    // println!("{}", I.embed_matrix(&m, 1, 1));
    // let a = m.inverse();
    //let s = m.det();
    let (q, r) = m1.qr_decomposition().unwrap();
    println!("Q:\n{},\nR:\n{}", q, r);
    println!("Product is:\n{}", q * r);
    println!("Rotation:\n{}", Matrix::rotation_y_3d(2.0));
    let fv1 = Vector {
        entries: vec![Fraction::new(1u64, 2u64); 3],
    };
    let fv2 = Vector {
        entries: vec![Fraction::new(3u64, 4u64); 3],
    };
    println!("Inner {}", fv1.inner(&fv2));
    let mut a = Matrix {
        rows: vec![vec![F::from(1), F::from(2)], vec![F::from(3), F::from(4)]],
    };
    //A[0][0] = Fraction::zero();
    let mut b = Matrix::filled(2, 2, 0.25_f64);
    b[0][0] = 0.1;
    println!("{}", a.inverse().unwrap());
}
