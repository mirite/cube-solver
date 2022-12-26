use crate::cube_parts::cube::Cube;
use crate::transforms::{get_all_turns, turn_cube, Turns};

pub fn solve(cube: &Cube) -> Vec<Turns> {
    let mut turns: Vec<Turns> = Vec::new();
    let mut working_cube = cube.clone();
    //step_1(&mut working_cube, &mut turns);
    turns
}
