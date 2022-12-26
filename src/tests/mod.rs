use crate::cube_parts::cube::{build_cube, Cube};

pub mod inverses;
pub mod multi_turns;
pub mod single_turns;
mod face;

pub fn create_vec(str: Vec<&str>) -> Vec<String> {
    let chars = str.iter().map(|&s| String::from(s));
    chars.collect()
}

pub fn solved_cube() -> Cube {
    let input = create_vec(vec![
        "", "w", "w", "w", "w", "w", "w", "w", "w", "w", "o", "o", "o", "o", "o", "o", "o", "o",
        "o", "y", "y", "y", "y", "y", "y", "y", "y", "y", "r", "r", "r", "r", "r", "r", "r", "r",
        "r", "g", "g", "g", "g", "g", "g", "g", "g", "g", "b", "b", "b", "b", "b", "b", "b", "b",
        "b",
    ]);
    build_cube(&input)
}
