use crate::cube_parts::cube::build_cube;
use crate::solver::solve;
use crate::step_1;
use crate::step_1::{is_solved, step_1};
use crate::tests::{create_vec, create_vec_from_pretty, solved_cube, test_cube};
use crate::transforms::{turn_cube, Turns};

fn test_solve(stating_map: Vec<&str>) -> () {
    // let mut cube = build_cube(&create_vec(stating_map));
    // let mut turns: Vec<Turns> = Vec::new();
    // step_1(&mut cube, &mut turns);
    //
    // assert!(cube.white.m.c, 'w', "Missing middle {}", cube.white);
}

#[test]
fn solved_cube_is_complete() {
    let cube = solved_cube();
    assert_eq!(step_1::is_solved(&cube), true);
}

#[test]
fn test_cube_is_incomplete() {
    let cube = test_cube();
    assert_eq!(step_1::is_solved(&cube), false);
}

#[test]
fn solve_1() {
    let result = vec![
        ["r", "y", "y", "b", "w", "b", "o", "y", "y"],
        ["r", "r", "o", "y", "o", "b", "b", "o", "o"],
        ["w", "w", "w", "b", "y", "g", "b", "o", "y"],
        ["y", "r", "r", "g", "r", "g", "b", "w", "w"],
        ["g", "g", "w", "w", "g", "w", "r", "r", "b"],
        ["g", "o", "g", "r", "b", "y", "g", "o", "o"],
    ];

    test_solve(create_vec_from_pretty(result));
}
