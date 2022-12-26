use crate::cube_parts::cube::{build_cube, test_cube};
use crate::tests::{create_vec, create_vec_from_pretty, solved_cube};
use crate::transforms::{turn_cube, Turns};

fn test_multi(result_map: Vec<&str>, turns: Vec<Turns>) -> () {
    let mut cube = solved_cube();
    let expected_result_cube = build_cube(&create_vec(result_map));

    for turn in turns {
        println!("{}\n\n", cube);
        cube = turn_cube(turn, cube);
        println!("{}\n\n", cube);
        test_cube(&cube);
    }
    println!("{}\n\n", cube);

    assert_eq!(
        cube, expected_result_cube,
        "\nActual Cube:\n{}\n\nExpected Cube:\n{}\n",
        cube, expected_result_cube
    );
}

#[test]
fn multi_1() {
    let result = vec![
        "", "o", "w", "r", "o", "w", "r", "g", "b", "r", "r", "o", "w", "y", "o", "w", "y", "g",
        "b", "g", "y", "b", "o", "y", "b", "o", "g", "b", "w", "r", "g", "w", "r", "y", "o", "r",
        "y", "w", "g", "g", "w", "g", "g", "w", "r", "r", "y", "o", "o", "y", "b", "b", "y", "b",
        "b",
    ];
    let mut turns: Vec<Turns> = Vec::new();
    turns.push(Turns::F);
    turns.push(Turns::D);
    turns.push(Turns::R);
    turns.push(Turns::L);

    test_multi(result, turns);
}

#[test]
fn multi_2() {
    let result = vec![
        ["r", "w", "o", "r", "w", "o", "r", "g", "b"],
        ["w", "w", "w", "w", "o", "y", "g", "b", "y"],
        ["g", "g", "g", "b", "y", "y", "o", "o", "b"],
        ["b", "r", "w", "y", "r", "w", "o", "g", "g"],
        ["r", "o", "y", "o", "g", "y", "w", "g", "y"],
        ["b", "b", "o", "b", "b", "r", "r", "r", "y"],
    ];
    let mut turns: Vec<Turns> = Vec::new();
    turns.push(Turns::Fp);
    turns.push(Turns::Dp);
    turns.push(Turns::Rp);
    turns.push(Turns::Lp);
    turns.push(Turns::Bp);

    test_multi(create_vec_from_pretty(result), turns);
}
