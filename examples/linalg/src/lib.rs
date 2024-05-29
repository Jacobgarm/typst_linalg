use wasm_minimal_protocol::*;

mod matrix;
use matrix::*;

initiate_protocol!();

macro_rules! unary {
    ($name: tt, $content: expr) => {
        #[wasm_func]
        pub fn $name(arg: &[u8]) -> Vec<u8> {
            let mat = Matrix::from_bytes(arg);
            let res = $content;
            res.to_bytes()
        }
    };
}

#[wasm_func]
pub fn add(arg1: &[u8], arg2: &[u8]) -> Vec<u8> {
    let res = Matrix::from_bytes(arg1) + Matrix::from_bytes(arg2);
    res.to_bytes()
}

#[wasm_func]
pub fn mul(arg1: &[u8], arg2: &[u8]) -> Vec<u8> {
    let res = Matrix::from_bytes(arg1) * Matrix::from_bytes(arg2);
    res.to_bytes()
}

//unary!(neg, { -mat });

