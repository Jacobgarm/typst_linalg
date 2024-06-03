#![allow(dead_code)]
#![allow(non_snake_case)]
#![feature(min_specialization)]
use fraction::Fraction;
use num::complex::c64;

mod common;
mod convert;
mod matrix;
mod vector;

use matrix::*;
use num::Complex;
use vector::*;
type F = Fraction;

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
    let I: Matrix<f64> = Matrix::id(15);
    let A = Matrix::filled(12, 14, 1.0);
    let v = Vector {
        entries: vec![2.0; 4],
    };
    let n = Matrix { rows: vec![
        vec![c64(1.0, 2.0), c64(-4.0, 1.0)],
        vec![c64(0.1, 5.0), c64(3.0, 1.2)]
    ] };
    // println!("{}", I.embed_matrix(&m, 1, 1));
    // let a = m.inverse();
    //let s = m.det();
    // let (Q, R) = m.QR().unwrap();
    // println!("Q:\n{},\nR:\n{}", Q, R);
    // println!("Product is:\n{}", Q * R);
    println!("Rotation:\n{}", Matrix::rotation_y_3d(2.0));
    let fv1 = Vector {
        entries: vec![Fraction::new(1u64, 2u64); 3],
    };
    let fv2 = Vector {
        entries: vec![Fraction::new(3u64, 4u64); 3],
    };
    println!("Inner {}", fv1.inner(&fv2));
    let mut A = Matrix {
        rows: vec![vec![F::from(1), F::from(2)], vec![F::from(3), F::from(4)]],
    };
    //A[0][0] = Fraction::zero();
    let mut B = Matrix::filled(2, 2, 0.25_f64);
    B[0][0] = 0.1;
    println!("{}", A.inverse().unwrap());
    println!("REF is:\n{}", n.REF().0);
}
