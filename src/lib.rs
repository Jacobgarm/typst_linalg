#![allow(dead_code)]
#![allow(non_snake_case)]
#![feature(min_specialization)]
use wasm_minimal_protocol::*;

mod common;
mod convert;
mod matrix;
mod vector;

use convert::Convertable;
use matrix::*;
use vector::*;

initiate_protocol!();

macro_rules! unary {
    ($name: tt, $content: tt) => {
        #[wasm_func]
        pub fn $name(arg: &[u8]) -> Result<Vec<u8>, String> {
            let mat = Matrix::from_bytes(arg)?;
            let res = $content(mat);
            Ok(res.to_bytes())
        }
    };
}

macro_rules! unary_err {
    ($name: tt, $content: tt) => {
        #[wasm_func]
        pub fn $name(arg: &[u8]) -> Result<Vec<u8>, String> {
            let mat = Matrix::from_bytes(arg)?;
            let res = $content(mat)?;
            Ok(res.to_bytes())
        }
    };
}

macro_rules! binary {
    ($name: tt, $content: tt) => {
        #[wasm_func]
        pub fn $name(arg1: &[u8], arg2: &[u8]) -> Result<Vec<u8>, String> {
            let mat1 = Matrix::from_bytes(arg1)?;
            let mat2 = Matrix::from_bytes(arg2)?;
            let res = $content(mat1, mat2);
            Ok(res.to_bytes())
        }
    };
}

type RMatrix = Matrix<f64>;

unary!(neg, { |m: RMatrix| -m });
unary!(transpose, { |m: RMatrix| m.transpose() });
unary!(REF, { |m: RMatrix| m.REF().0 });
unary!(RREF, { |m: RMatrix| m.RREF() });
unary_err!(det, { |m: RMatrix| m.det() });
unary_err!(trace, { |m: RMatrix| m.trace() });
unary_err!(inverse, { |m: RMatrix| m.inverse() });
unary_err!(exp, { |m: RMatrix| m.exp() });

binary!(add, { |m1: RMatrix, m2: RMatrix| m1 + m2 });
binary!(sub, { |m1: RMatrix, m2: RMatrix| m1 - m2 });
binary!(mul, { |m1: RMatrix, m2: RMatrix| m1 * m2 });

#[wasm_func]
pub fn pow(mat_bytes: &[u8], pow_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mat = RMatrix::from_bytes(mat_bytes)?;
    let pow = i64::from_bytes(pow_bytes)?;
    let res = mat.powi(pow)?;
    Ok(res.to_bytes())
}

#[wasm_func]
pub fn rowswap(mat_bytes: &[u8], r1_bytes: &[u8], r2_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mat: RMatrix = Matrix::from_bytes(mat_bytes)?;
    let r1 = usize::from_bytes(r1_bytes)?;
    let r2 = usize::from_bytes(r2_bytes)?;
    let res = mat.rowswap(r1, r2)?;
    Ok(res.to_bytes())
}

#[wasm_func]
pub fn mul_vec(mat_bytes: &[u8], vec_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mat: RMatrix = Matrix::from_bytes(mat_bytes)?;
    let vec = Vector::from_bytes(vec_bytes)?;
    let res = mat.mul_vector(vec)?;
    Ok(res.to_bytes())
}
