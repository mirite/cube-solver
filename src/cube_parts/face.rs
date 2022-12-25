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

pub fn rotate_clockwise(
    mut working_face: &mut Face,
    mut top_edge: &mut Face,
    mut right_edge: &mut Face,
    mut down_edge: &mut Face,
    mut left_edge: &mut Face,
    orientation: Orientation,
) -> () {
    rotate_face_clockwise(&mut working_face);
    let temp_top_edge = top_edge.clone();
    let temp_right_edge = right_edge.clone();
    let temp_down_edge = down_edge.clone();
    let temp_left_edge = left_edge.clone();
    if orientation == Orientation::Y {
    } else if orientation == Orientation::Z {
        //Face, Back
        top_edge.b.l = temp_left_edge.b.r;
        top_edge.b.c = temp_left_edge.m.r;
        top_edge.b.r = temp_left_edge.t.r;
        right_edge.t.l = temp_top_edge.b.l;
        right_edge.m.l = temp_top_edge.b.c;
        right_edge.b.l = temp_top_edge.b.r;
        down_edge.t.l = temp_right_edge.t.l;
        down_edge.t.c = temp_right_edge.m.l;
        down_edge.t.r = temp_right_edge.b.l;
        left_edge.t.r = temp_down_edge.t.l;
        left_edge.m.r = temp_down_edge.t.c;
        left_edge.b.r = temp_down_edge.t.r;
    } else if orientation == Orientation::X {
        //Right, Left
        top_edge.t.r = temp_left_edge.b.r;
        top_edge.m.r = temp_left_edge.m.r;
        top_edge.b.r = temp_left_edge.t.r;
        right_edge.t.l = temp_top_edge.b.l;
        right_edge.m.l = temp_top_edge.b.c;
        right_edge.b.l = temp_top_edge.b.r;
        down_edge.t.r = temp_right_edge.t.l;
        down_edge.m.r = temp_right_edge.m.l;
        down_edge.b.r = temp_right_edge.b.l;
        left_edge.t.r = temp_down_edge.t.l;
        left_edge.m.r = temp_down_edge.t.c;
        left_edge.b.r = temp_down_edge.t.r;
    }
}

fn rotate_face_clockwise(working_face: &mut Face) {
    let temp_face = working_face.clone();
    working_face.t.l = temp_face.b.l;
    working_face.t.c = temp_face.m.l;
    working_face.t.r = temp_face.t.l;
    working_face.m.l = temp_face.b.c;
    working_face.m.r = temp_face.t.c;
    working_face.b.l = temp_face.b.r;
    working_face.b.c = temp_face.m.r;
    working_face.b.r = temp_face.t.r;
}

pub fn rotate_counter_clockwise(
    mut working_face: &mut Face,
    mut top_edge: &mut Face,
    mut right_edge: &mut Face,
    mut down_edge: &mut Face,
    mut left_edge: &mut Face,
    orientation: Orientation,
) -> () {
    rotate_face_counter_clockwise(&mut working_face);
    let temp_top_edge = top_edge.clone();
    let temp_right_edge = right_edge.clone();
    let temp_down_edge = down_edge.clone();
    let temp_left_edge = left_edge.clone();
    if orientation == Orientation::Y {
    } else if orientation == Orientation::Z {
        top_edge.b.l = temp_right_edge.t.l;
        top_edge.b.c = temp_right_edge.m.l;
        top_edge.b.r = temp_right_edge.b.l;
        right_edge.t.l = temp_down_edge.t.l;
        right_edge.m.l = temp_down_edge.t.c;
        right_edge.b.l = temp_down_edge.t.r;
        down_edge.t.l = temp_left_edge.t.r;
        down_edge.t.c = temp_left_edge.m.r;
        down_edge.t.r = temp_left_edge.b.r;
        left_edge.t.r = temp_top_edge.b.l;
        left_edge.m.r = temp_top_edge.b.c;
        left_edge.b.r = temp_top_edge.b.r;
    } else if orientation == Orientation::X {
        top_edge.t.r = temp_right_edge.t.l;
        top_edge.m.r = temp_right_edge.m.l;
        top_edge.b.r = temp_right_edge.b.l;
        right_edge.t.l = temp_down_edge.t.l;
        right_edge.m.l = temp_down_edge.t.c;
        right_edge.b.l = temp_down_edge.t.r;
        down_edge.t.r = temp_left_edge.t.r;
        down_edge.m.r = temp_left_edge.m.r;
        down_edge.b.r = temp_left_edge.b.r;
        left_edge.t.r = temp_top_edge.b.l;
        left_edge.m.r = temp_top_edge.b.c;
        left_edge.b.r = temp_top_edge.b.r;
    }
}

fn rotate_face_counter_clockwise(working_face: &mut Face) {
    let temp_face = working_face.clone();
    working_face.t.l = temp_face.t.r;
    working_face.t.c = temp_face.m.r;
    working_face.t.r = temp_face.b.r;
    working_face.m.l = temp_face.t.c;
    working_face.m.r = temp_face.b.c;
    working_face.b.l = temp_face.t.l;
    working_face.b.c = temp_face.m.l;
    working_face.b.r = temp_face.b.l;
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
