use crate::cube_parts::cube::{build_cube, create_vec, create_vec_from_pretty, Cube};

mod face;
pub mod inverses;
pub mod multi_turns;
pub mod single_turns;
mod step_1_solves;

pub fn test_cube() -> Cube {
    let input = vec![
        ["a", "c", "d", "e", "w", "f", "h", "i", "j"],
        ["k", "l", "m", "n", "o", "p", "q", "s", "t"],
        ["u", "v", "x", "z", "y", "1", "2", "3", "4"],
        ["5", "6", "7", "8", "r", "9", "0", "A", "B"],
        ["C", "D", "E", "F", "g", "H", "I", "J", "L"],
        ["L", "M", "N", "P", "b", "Q", "R", "S", "T"],
    ];
    let intermediate = create_vec_from_pretty(input);
    build_cube(&create_vec(intermediate))
}
