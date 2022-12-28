use crate::cube_parts::cube::{build_cube, solved_cube, test_cube, Cube};
use crate::transforms::{turn_cube, Turns};
use std::env;

#[cfg(test)]
pub mod tests;

pub mod cube_parts;
mod solver;
mod step_1;
mod transforms;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cube = build_cube(&args);
    test_cube(&cube);
    println!("Initial");
    println!("{}", cube);

    cube = turn_cube(Turns::F, cube);
    cube = turn_cube(Turns::R, cube);
    println!("{}", cube);

    println!("Test");
    test_cube(&cube);
}
