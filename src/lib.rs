use wasm_minimal_protocol::*;

mod common;
mod convert;
mod matrix;
mod vector;

use convert::Convertable;
use matrix::*;

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

unary!(neg, { |m: Matrix| -m });
unary!(transpose, { |m: Matrix| m.transpose() });
unary!(REF, { |m: Matrix| m.REF().0 });
unary!(RREF, { |m: Matrix| m.RREF() });
unary_err!(det, { |m: Matrix| m.det() });
unary_err!(trace, { |m: Matrix| m.trace() });
unary_err!(inverse, { |m: Matrix| m.inverse() });

binary!(add, { |m1: Matrix, m2: Matrix| m1 + m2 });
binary!(sub, { |m1: Matrix, m2: Matrix| m1 - m2 });
binary!(mul, { |m1: Matrix, m2: Matrix| m1 * m2 });

#[wasm_func]
pub fn rowswap(mat_bytes: &[u8], r1_bytes: &[u8], r2_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mat = Matrix::from_bytes(mat_bytes)?;
    let r1 = usize::from_bytes(r1_bytes)?;
    let r2 = usize::from_bytes(r2_bytes)?;
    let res = mat.rowswap(r1, r2)?;
    Ok(res.to_bytes())
}
