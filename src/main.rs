use crate::cube_parts::cube::{build_cube, test_cube};
use crate::transforms::{turn_cube, Turns};
use std::env;

pub mod cube_parts;
mod transforms;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cube = build_cube(&args);
    test_cube(&cube);
    println!("Initial");
    println!("{}", cube);

    println!("F turn");
    cube = turn_cube(Turns::Fp, cube);
    println!("{}", cube);

    println!("F' turn");
    cube = turn_cube(Turns::Fp, cube);
    println!("{}", cube);

    println!("Test");
    test_cube(&cube);
}
