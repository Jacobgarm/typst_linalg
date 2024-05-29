use wasm_minimal_protocol::*;

initiate_protocol!();

#[wasm_func]
pub fn add(arg: &[u8]) -> Vec<u8> {
    return arg.to_vec();
}