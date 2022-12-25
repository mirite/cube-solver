use crate::cube_parts::cube::build_cube;
use crate::tests::{create_vec, solved_cube};
use crate::transforms::{turn_cube, Turns};

fn test_single(result_map: Vec<&str>, turn: Turns) -> () {
    let mut cube = solved_cube();
    let expected_result_cube = build_cube(&create_vec(result_map));

    cube = turn_cube(turn, cube);
    assert_eq!(
        cube, expected_result_cube,
        "\nActual Cube:\n{}\n\nExpected Cube:\n{}\n",
        cube, expected_result_cube
    );
}

#[test]
fn face_single() {
    let result = vec![
        "", "w", "w", "w", "w", "w", "w", "w", "w", "w", "o", "o", "o", "o", "o", "o", "g", "g",
        "g", "y", "y", "y", "y", "y", "y", "y", "y", "y", "b", "b", "b", "r", "r", "r", "r", "r",
        "r", "g", "g", "r", "g", "g", "r", "g", "g", "r", "o", "b", "b", "o", "b", "b", "o", "b",
        "b",
    ];
    test_single(result, Turns::F);
}

#[test]
fn face_counter_single() {
    let result = vec![
        "", "w", "w", "w", "w", "w", "w", "w", "w", "w", "o", "o", "o", "o", "o", "o", "b", "b",
        "b", "y", "y", "y", "y", "y", "y", "y", "y", "y", "g", "g", "g", "r", "r", "r", "r", "r",
        "r", "g", "g", "o", "g", "g", "o", "g", "g", "o", "r", "b", "b", "r", "b", "b", "r", "b",
        "b",
    ];
    test_single(result, Turns::Fp);
}

#[test]
fn right_single() {
    let result = vec![
        "", "w", "w", "r", "w", "w", "r", "w", "w", "r", "o", "o", "w", "o", "o", "w", "o", "o",
        "w", "o", "y", "y", "o", "y", "y", "o", "y", "y", "r", "r", "y", "r", "r", "y", "r", "r",
        "y", "g", "g", "g", "g", "g", "g", "g", "g", "g", "b", "b", "b", "b", "b", "b", "b", "b",
        "b",
    ];
    test_single(result, Turns::R);
}

#[test]
fn right_counter_single() {
    let result = vec![
        "", "w", "w", "o", "w", "w", "o", "w", "w", "o", "o", "o", "y", "o", "o", "y", "o", "o",
        "y", "r", "y", "y", "r", "y", "y", "r", "y", "y", "r", "r", "w", "r", "r", "w", "r", "r",
        "w", "g", "g", "g", "g", "g", "g", "g", "g", "g", "b", "b", "b", "b", "b", "b", "b", "b",
        "b",
    ];
    test_single(result, Turns::Rp);
}