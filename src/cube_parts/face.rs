use super::row::Row;
use super::square::parse_square;
use std::fmt::{Display, Formatter};
use crate::cube_parts::cube::{Counts, Cube};

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

pub fn get_opposite_face(cube: &Cube, colour: char) -> &Face {
    match colour {
        'w' => &cube.yellow,
        'y' => &cube.white,
        'b' => &cube.green,
        'g' => &cube.blue,
        'o' => &cube.red,
        'r' => &cube.orange,
        _ => panic!("Invalid side provided {}", colour)
    }
}

pub fn get_adjacent_faces(cube: &Cube, colour: char) -> (&Face, &Face, &Face, &Face) {
    match colour {
        'w' => (&cube.red, &cube.green, &cube.orange, &cube.blue),
        'r' => (&cube.white, &cube.blue, &cube.yellow, &cube.green),
        'y' => (&cube.red, &cube.blue, &cube.orange, &cube.green),
        'o' => (&cube.yellow, &cube.blue, &cube.white, &cube.green),
        'b' => (&cube.white, &cube.orange, &cube.yellow, &cube.red),
        'g' => (&cube.red, &cube.yellow, &cube.orange, &cube.white),
        _ => panic!("Invalid side provided {}", colour)
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
            _ => panic!("Invalid colour found {}", value)
        }
    }
    counts
}
