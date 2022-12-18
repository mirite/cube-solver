use crate::cube_parts::cube::{build_cube, test_cube};
use crate::cube_parts::face::get_opposite_face;
use std::env;

pub mod cube_parts;
fn main() {
    let args: Vec<String> = env::args().collect();
    let cube = build_cube(&args);
    test_cube(&cube);
    println!("{}", cube);
    let back = get_opposite_face(&cube, 'w');
    println!("{}", back);
}
