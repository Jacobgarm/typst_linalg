#![allow(dead_code)]
#![feature(min_specialization)]
use fraction::Fraction;
use num::complex::c64;

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
    let v1 = Vector {
        entries: vec![1.0, 0.0],
    };
    let v2 = Vector {
        entries: vec![0.0, 1.0],
    };
    println!("Angle is {}", v1.angle_with(&v2).unwrap());

    let n = Matrix {
        rows: vec![
            vec![c64(1.0, 2.0), c64(-4.0, 1.0)],
            vec![c64(0.1, 5.0), c64(3.0, 1.2)],
            vec![c64(1.0, 2.0), c64(-4.0, 1.0)],
            vec![c64(0.1, 5.0), c64(3.0, 1.2)],
        ],
    };

    let v = Matrix::vandermonde(&[F::from(2), F::new(1u64, 2u64)], 4);
    println!("Vandermonde\n{}", v);
    let u = Matrix {
        rows: vec![vec![0.3, 0.0], vec![0.2, 0.1]],
    };
    println!("{}", u.is_lower_triangular())
}
