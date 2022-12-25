use crate::tests::solved_cube;
use crate::transforms::{turn_cube, Turns};

fn test_inverse(clockwise: Turns, counter_clockwise: Turns) -> () {
    let mut cube = solved_cube();
    let expected_result_cube = solved_cube();

    cube = turn_cube(clockwise, cube);
    cube = turn_cube(counter_clockwise, cube);
    assert_eq!(
        cube, expected_result_cube,
        "\nActual Cube:\n{}\n\nExpected Cube:\n{}\n",
        cube, expected_result_cube
    );
}

#[test]
fn right_inverses() {
    test_inverse(Turns::R, Turns::Rp);
}

#[test]
fn left_inverses() {
    test_inverse(Turns::L, Turns::Lp);
}

#[test]
fn front_inverses() {
    test_inverse(Turns::F, Turns::Fp);
}

#[test]
fn back_inverses() {
    test_inverse(Turns::B, Turns::Bp);
}

#[test]
fn up_inverses() {
    test_inverse(Turns::U, Turns::Up);
}

#[test]
fn down_inverses() {
    test_inverse(Turns::D, Turns::Dp);
}
