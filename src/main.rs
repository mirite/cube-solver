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
    cube = turn_cube(Turns::R, cube);
    println!("{}", cube);

    println!("Test");
    test_cube(&cube);
}

#[cfg(test)]
mod tests {
    use crate::cube_parts::cube::{build_cube, Cube};
    use crate::transforms::{turn_cube, Turns};

    fn create_vec(str: Vec<&str>) -> Vec<String> {
        let chars = str.iter().map(|&s| String::from(s));
        chars.collect()
    }

    fn solved_cube() -> Cube {
        let input = create_vec(vec![
            "", "w", "w", "w", "w", "w", "w", "w", "w", "w", "o", "o", "o", "o", "o", "o", "o",
            "o", "o", "y", "y", "y", "y", "y", "y", "y", "y", "y", "r", "r", "r", "r", "r", "r",
            "r", "r", "r", "g", "g", "g", "g", "g", "g", "g", "g", "g", "b", "b", "b", "b", "b",
            "b", "b", "b", "b",
        ]);
        build_cube(&input)
    }

    fn test_inverse(clockwise: Turns, counter_clockwise: Turns) -> (Cube, Cube) {
        let mut cube = solved_cube();
        let expected_result_cube = solved_cube();

        cube = turn_cube(clockwise, cube);
        cube = turn_cube(counter_clockwise, cube);
        (cube, expected_result_cube)
    }

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
    fn right_inverses() {
        let (cube, expected_result_cube) = test_inverse(Turns::R, Turns::Rp);
        assert_eq!(cube, expected_result_cube);
    }

    #[test]
    fn left_inverses() {
        let (cube, expected_result_cube) = test_inverse(Turns::L, Turns::Lp);
        assert_eq!(cube, expected_result_cube);
    }

    #[test]
    fn front_inverses() {
        let (cube, expected_result_cube) = test_inverse(Turns::F, Turns::Fp);
        assert_eq!(cube, expected_result_cube);
    }

    #[test]
    fn back_inverses() {
        let (cube, expected_result_cube) = test_inverse(Turns::B, Turns::Bp);
        assert_eq!(cube, expected_result_cube);
    }

    #[test]
    fn up_inverses() {
        let (cube, expected_result_cube) = test_inverse(Turns::U, Turns::Up);
        assert_eq!(cube, expected_result_cube);
    }

    #[test]
    fn down_inverses() {
        let (cube, expected_result_cube) = test_inverse(Turns::D, Turns::Dp);
        assert_eq!(cube, expected_result_cube);
    }

    #[test]
    fn face_single() {
        let result = vec![
            "", "w", "w", "w", "w", "w", "w", "w", "w", "w", "o", "o", "o", "o", "o", "o", "g",
            "g", "g", "y", "y", "y", "y", "y", "y", "y", "y", "y", "b", "b", "b", "r", "r", "r",
            "r", "r", "r", "g", "g", "r", "g", "g", "r", "g", "g", "r", "o", "b", "b", "o", "b",
            "b", "o", "b", "b",
        ];
        test_single(result, Turns::F);
    }

    #[test]
    fn face_counter_single() {
        let result = vec![
            "", "w", "w", "w", "w", "w", "w", "w", "w", "w", "o", "o", "o", "o", "o", "o", "b",
            "b", "b", "y", "y", "y", "y", "y", "y", "y", "y", "y", "g", "g", "g", "r", "r", "r",
            "r", "r", "r", "g", "g", "o", "g", "g", "o", "g", "g", "o", "r", "b", "b", "r", "b",
            "b", "r", "b", "b",
        ];
        test_single(result, Turns::Fp);
    }

    #[test]
    fn right_single() {
        let result = vec![
            "", "w", "w", "r", "w", "w", "r", "w", "w", "r", "o", "o", "w", "o", "o", "w", "o",
            "o", "w", "y", "y", "o", "y", "y", "o", "y", "y", "o", "r", "r", "y", "r", "r", "y",
            "r", "r", "y", "g", "g", "g", "g", "g", "g", "g", "g", "g", "b", "b", "b", "b", "b",
            "b", "b", "b", "b",
        ];
        test_single(result, Turns::R);
    }

    #[test]
    fn right_counter_single() {
        let result = vec![
            "", "w", "w", "o", "w", "w", "o", "w", "w", "o", "o", "o", "y", "o", "o", "y", "o",
            "o", "y", "o", "y", "y", "o", "y", "y", "o", "y", "y", "r", "r", "w", "r", "r", "w",
            "r", "r", "w", "g", "g", "g", "g", "g", "g", "g", "g", "g", "b", "b", "b", "b", "b",
            "b", "b", "b", "b",
        ];
        test_single(result, Turns::Rp);
    }
}
