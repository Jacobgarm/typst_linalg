use wasm_minimal_protocol::*;

mod matrix;
mod vector;
use matrix::*;

initiate_protocol!();

macro_rules! unary {
    ($name: tt, $content: tt) => {
        #[wasm_func]
        pub fn $name(arg: &[u8]) -> Vec<u8> {
            let mat = Matrix::from_bytes(arg);
            let res = $content(mat);
            res.to_bytes()
        }
    };
}

macro_rules! binary {
    ($name: tt, $content: tt) => {
        #[wasm_func]
        pub fn $name(arg1: &[u8], arg2: &[u8]) -> Vec<u8> {
            let mat1 = Matrix::from_bytes(arg1);
            let mat2 = Matrix::from_bytes(arg2);
            let res = $content(mat1, mat2);
            res.to_bytes()
        }
    };
}

unary!(neg, { |m: Matrix| -m });

binary!(add, { |m1: Matrix, m2: Matrix| m1 + m2 });
binary!(sub, { |m1: Matrix, m2: Matrix| m1 - m2 });
binary!(mul, { |m1: Matrix, m2: Matrix| m1 * m2 });

