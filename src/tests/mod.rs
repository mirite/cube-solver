use crate::cube_parts::cube::{build_cube, Cube};

mod face;
pub mod inverses;
pub mod multi_turns;
pub mod single_turns;

pub fn create_vec(str: Vec<&str>) -> Vec<String> {
    let chars = str.iter().map(|&s| String::from(s));
    chars.collect()
}

pub fn create_vec_from_pretty(str: Vec<[&str; 9]>) -> Vec<&str> {
    let mut chars: Vec<&str> = Vec::new();
    chars.push("");
    for group in str {
        for c in group {
            chars.push(c);
        }
    }
    chars
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

pub fn test_cube() -> Cube {
    let input = create_vec(vec![
        "", "a", "c", "d", "e", "w", "f", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "s",
        "t", "u", "v", "x", "y", "z", "1", "2", "3", "4", "5", "6", "7", "8", "r", "9", "0", "A",
        "B", "C", "D", "E", "F", "g", "H", "I", "J", "L", "L", "M", "N", "P", "b", "Q", "R", "S",
        "T",
    ]);
    build_cube(&input)
}
