use wasm_minimal_protocol::*;

mod matrix;
use matrix::*;

initiate_protocol!();

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

