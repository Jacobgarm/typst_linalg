use wasm_minimal_protocol::*;

mod matrix;
use matrix::*;

initiate_protocol!();

#[wasm_func]
pub fn add(arg: &[u8]) -> Vec<u8> {
    return [arg[0]+1].to_vec();
}