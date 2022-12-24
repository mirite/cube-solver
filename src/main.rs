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

    cube = turn_cube(Turns::F, cube);
    cube = turn_cube(Turns::F, cube);
    cube = turn_cube(Turns::F, cube);
    cube = turn_cube(Turns::F, cube);
    cube = turn_cube(Turns::Fp, cube);
    cube = turn_cube(Turns::Fp, cube);
    cube = turn_cube(Turns::Fp, cube);
    cube = turn_cube(Turns::Fp, cube);
    cube = turn_cube(Turns::R, cube);
    cube = turn_cube(Turns::R, cube);
    cube = turn_cube(Turns::R, cube);
    cube = turn_cube(Turns::R, cube);
    cube = turn_cube(Turns::Rp, cube);
    cube = turn_cube(Turns::Rp, cube);
    cube = turn_cube(Turns::Rp, cube);
    cube = turn_cube(Turns::Rp, cube);
    cube = turn_cube(Turns::L, cube);
    cube = turn_cube(Turns::L, cube);
    cube = turn_cube(Turns::L, cube);
    cube = turn_cube(Turns::L, cube);
    cube = turn_cube(Turns::Lp, cube);
    cube = turn_cube(Turns::Lp, cube);
    cube = turn_cube(Turns::Lp, cube);
    cube = turn_cube(Turns::Lp, cube);
    cube = turn_cube(Turns::U, cube);
    cube = turn_cube(Turns::U, cube);
    cube = turn_cube(Turns::U, cube);
    cube = turn_cube(Turns::U, cube);
    cube = turn_cube(Turns::Up, cube);
    cube = turn_cube(Turns::Up, cube);
    cube = turn_cube(Turns::Up, cube);
    cube = turn_cube(Turns::Up, cube);
    cube = turn_cube(Turns::D, cube);
    cube = turn_cube(Turns::D, cube);
    cube = turn_cube(Turns::D, cube);
    cube = turn_cube(Turns::D, cube);
    cube = turn_cube(Turns::Dp, cube);
    cube = turn_cube(Turns::Dp, cube);
    cube = turn_cube(Turns::Dp, cube);
    cube = turn_cube(Turns::Dp, cube);
    cube = turn_cube(Turns::B, cube);
    cube = turn_cube(Turns::B, cube);
    cube = turn_cube(Turns::B, cube);
    cube = turn_cube(Turns::B, cube);
    cube = turn_cube(Turns::Bp, cube);
    cube = turn_cube(Turns::Bp, cube);
    cube = turn_cube(Turns::B, cube);
    cube = turn_cube(Turns::Bp, cube);

    println!("Test");
    test_cube(&cube);
}
