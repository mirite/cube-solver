use crate::cube_parts::cube::{build_cube, create_vec, create_vec_from_pretty, solved_cube};
use crate::tests::test_cube;
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

fn test_single_advanced(result_map: Vec<[&str; 9]>, turn: Turns) -> () {
    let mut cube = test_cube();
    println!("{}", cube);
    let expected_result_cube = build_cube(&create_vec(create_vec_from_pretty(result_map)));

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
        ["h", "e", "a", "i", "w", "c", "j", "f", "d"],
        ["k", "l", "m", "n", "o", "p", "L", "H", "E"],
        ["u", "v", "x", "z", "y", "1", "2", "3", "4"],
        ["R", "P", "L", "8", "r", "9", "0", "A", "B"],
        ["C", "D", "5", "F", "g", "6", "I", "J", "7"],
        ["q", "M", "N", "s", "b", "Q", "t", "S", "T"],
    ];

    test_single_advanced(result, Turns::F);
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
        ["a", "c", "7", "e", "w", "9", "h", "i", "B"],
        ["k", "l", "d", "n", "o", "f", "q", "s", "j"],
        ["t", "v", "x", "p", "y", "1", "m", "3", "4"],
        ["5", "6", "2", "8", "r", "z", "0", "A", "u"],
        ["C", "D", "E", "F", "g", "H", "I", "J", "L"],
        ["R", "P", "L", "S", "b", "M", "T", "Q", "N"],
    ];

    test_single_advanced(result, Turns::R);
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

#[test]
fn left_single() {
    let result = vec![
        "", "o", "w", "w", "o", "w", "w", "o", "w", "w", "y", "o", "o", "y", "o", "o", "y", "o",
        "o", "y", "y", "r", "y", "y", "r", "y", "y", "r", "w", "r", "r", "w", "r", "r", "w", "r",
        "r", "g", "g", "g", "g", "g", "g", "g", "g", "g", "b", "b", "b", "b", "b", "b", "b", "b",
        "b",
    ];
    test_single(result, Turns::L);
}

#[test]
fn left_counter_single() {
    let result = vec![
        "", "r", "w", "w", "r", "w", "w", "r", "w", "w", "w", "o", "o", "w", "o", "o", "w", "o",
        "o", "y", "y", "o", "y", "y", "o", "y", "y", "o", "y", "r", "r", "y", "r", "r", "y", "r",
        "r", "g", "g", "g", "g", "g", "g", "g", "g", "g", "b", "b", "b", "b", "b", "b", "b", "b",
        "b",
    ];
    test_single(result, Turns::Lp);
}

#[test]
fn up_single() {
    let result = vec![
        ["L", "M", "N", "e", "w", "f", "h", "i", "j"],
        ["q", "n", "k", "s", "o", "l", "t", "p", "m"],
        ["C", "D", "E", "z", "y", "1", "2", "3", "4"],
        ["5", "6", "7", "8", "r", "9", "0", "A", "B"],
        ["a", "c", "d", "F", "g", "H", "I", "J", "L"],
        ["u", "v", "x", "P", "b", "Q", "R", "S", "T"],
    ];

    test_single_advanced(result, Turns::U);
}

#[test]
fn up_counter_single() {
    let result = vec![
        "", "g", "g", "g", "w", "w", "w", "w", "w", "w", "o", "o", "o", "o", "o", "o", "o", "o",
        "o", "b", "b", "b", "y", "y", "y", "y", "y", "y", "r", "r", "r", "r", "r", "r", "r", "r",
        "r", "y", "y", "y", "g", "g", "g", "g", "g", "g", "w", "w", "w", "b", "b", "b", "b", "b",
        "b",
    ];
    test_single(result, Turns::Up);
}

#[test]
fn down_single() {
    let result = vec![
        "", "w", "w", "w", "w", "w", "w", "b", "b", "b", "o", "o", "o", "o", "o", "o", "o", "o",
        "o", "y", "y", "y", "y", "y", "y", "g", "g", "g", "r", "r", "r", "r", "r", "r", "r", "r",
        "r", "g", "g", "g", "g", "g", "g", "w", "w", "w", "b", "b", "b", "b", "b", "b", "y", "y",
        "y",
    ];
    test_single(result, Turns::D);
}

#[test]
fn down_counter_single() {
    let result = vec![
        "", "w", "w", "w", "w", "w", "w", "g", "g", "g", "o", "o", "o", "o", "o", "o", "o", "o",
        "o", "y", "y", "y", "y", "y", "y", "b", "b", "b", "r", "r", "r", "r", "r", "r", "r", "r",
        "r", "g", "g", "g", "g", "g", "g", "y", "y", "y", "b", "b", "b", "b", "b", "b", "w", "w",
        "w",
    ];
    test_single(result, Turns::Dp);
}

#[test]
fn back_single() {
    let result = vec![
        "", "w", "w", "w", "w", "w", "w", "w", "w", "w", "g", "g", "g", "o", "o", "o", "o", "o",
        "o", "y", "y", "y", "y", "y", "y", "y", "y", "y", "r", "r", "r", "r", "r", "r", "b", "b",
        "b", "r", "g", "g", "r", "g", "g", "r", "g", "g", "b", "b", "o", "b", "b", "o", "b", "b",
        "o",
    ];
    test_single(result, Turns::B);
}

#[test]
fn back_counter_single() {
    let result = vec![
        "", "w", "w", "w", "w", "w", "w", "w", "w", "w", "b", "b", "b", "o", "o", "o", "o", "o",
        "o", "y", "y", "y", "y", "y", "y", "y", "y", "y", "r", "r", "r", "r", "r", "r", "g", "g",
        "g", "o", "g", "g", "o", "g", "g", "o", "g", "g", "b", "b", "r", "b", "b", "r", "b", "b",
        "r",
    ];
    test_single(result, Turns::Bp);
}
