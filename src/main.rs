use crate::cube_parts::cube::{build_cube, test_cube};
use std::env;

pub mod cube_parts;
fn main() {
    let args: Vec<String> = env::args().collect();
    let cube = build_cube(&args);
    test_cube(&cube);
    println!("{}", cube);
}
