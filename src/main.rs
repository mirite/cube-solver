use crate::cube_parts::cube::{build_cube, test_cube};
use crate::cube_parts::face::get_opposite_face;
use crate::transforms::{turn_cube, Turns};
use std::env;

pub mod cube_parts;
mod transforms;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cube = build_cube(&args);
    test_cube(&cube);
    println!("{}", cube);
    let back = get_opposite_face(&cube, 'w');
    println!("{}", back);
    cube = turn_cube(Turns::F, cube);
    println!("{}", cube);
    test_cube(&cube);
}
