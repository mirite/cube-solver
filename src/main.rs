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

    #[test]
    fn it_works() {
        let result = vec![
            "", "w", "w", "r", "w", "w", "r", "w", "w", "r", "o", "o", "w", "o", "o", "w", "o",
            "o", "w", "y", "y", "o", "y", "y", "o", "y", "y", "o", "r", "r", "y", "r", "r", "y",
            "r", "r", "y", "g", "g", "g", "g", "g", "g", "g", "g", "g", "b", "b", "b", "b", "b",
            "b", "b", "b", "b",
        ];

        let mut cube = solved_cube();
        let expected_result_cube = build_cube(&create_vec(result));

        cube = turn_cube(Turns::R, cube);

        assert_eq!(cube, expected_result_cube);
    }
}
