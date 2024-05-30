mod convert;
mod matrix;
mod vector;

use matrix::*;
fn main() {
    let m = Matrix {
        rows: vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
    };
    let (a, _) = m.REF();
    //let s = m.det();
    dbg!(a);
}
