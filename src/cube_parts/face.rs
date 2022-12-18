use super::row::Row;
use super::square::parse_square;
use std::fmt::{Display, Formatter};
use crate::cube_parts::cube::Cube;

pub struct Face {
    pub t: Row,
    pub m: Row,
    pub b: Row,
}

pub fn build_side(args: &Vec<String>, start: usize) -> Face {
    Face {
        t: Row {
            l: parse_square(args, start),
            c: parse_square(args, start + 1),
            r: parse_square(args, start + 2),
        },
        m: Row {
            l: parse_square(args, start + 3),
            c: parse_square(args, start + 4),
            r: parse_square(args, start + 5),
        },
        b: Row {
            l: parse_square(args, start + 6),
            c: parse_square(args, start + 7),
            r: parse_square(args, start + 8),
        },
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, " _ _ _ \n{}\n{}\n{}\n - - - ", self.t, self.m, self.b)
    }
}

pub fn get_opposite_face(face: &Face, cube: &Cube) -> &Face {
    match face.m.c {
        'w' => &cube.yellow,
        'y' => &cube.white,
        'b' => &cube.green,
        'g' => &cube.blue,
        'o' => &cube.red,
        'r' => &cube.orange,
        _ => panic!("Invalid side provided {}",face.m.c)
    }
}
