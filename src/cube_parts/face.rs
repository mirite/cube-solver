use super::row::Row;
use super::square::parse_square;
use crate::cube_parts::cube::{Counts, Cube};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Face {
    pub t: Row,
    pub m: Row,
    pub b: Row,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Orientation {
    X,
    Y,
    Z,
}

pub fn rotate_face_clockwise(mut working_face: Face) -> Face {
    let temp_face = working_face.clone();
    working_face.t.l = temp_face.b.l;
    working_face.t.c = temp_face.m.l;
    working_face.t.r = temp_face.t.l;
    working_face.m.l = temp_face.b.c;
    working_face.m.r = temp_face.t.c;
    working_face.b.l = temp_face.b.r;
    working_face.b.c = temp_face.m.r;
    working_face.b.r = temp_face.t.r;
    working_face
}

pub fn rotate_face_counter_clockwise(mut working_face: Face) -> Face {
    let temp_face = working_face.clone();
    working_face.t.l = temp_face.t.r;
    working_face.t.c = temp_face.m.r;
    working_face.t.r = temp_face.b.r;
    working_face.m.l = temp_face.t.c;
    working_face.m.r = temp_face.b.c;
    working_face.b.l = temp_face.t.l;
    working_face.b.c = temp_face.m.l;
    working_face.b.r = temp_face.b.l;
    working_face
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

pub fn test_face(face: &Face, mut counts: Counts, expected_colour: char) -> Counts {
    assert_eq!(face.m.c, expected_colour);
    let mut face_values = [' '; 9];
    face_values[0] = face.t.l;
    face_values[1] = face.t.c;
    face_values[2] = face.t.r;
    face_values[3] = face.m.l;
    face_values[4] = face.m.c;
    face_values[5] = face.m.r;
    face_values[6] = face.b.l;
    face_values[7] = face.b.c;
    face_values[8] = face.b.r;

    for value in face_values {
        match value {
            'y' => counts.y += 1,
            'b' => counts.b += 1,
            'g' => counts.g += 1,
            'r' => counts.r += 1,
            'w' => counts.w += 1,
            'o' => counts.o += 1,
            _ => panic!("Invalid colour found {}", value),
        }
    }
    counts
}
