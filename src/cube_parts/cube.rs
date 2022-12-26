use super::face::build_side;
use crate::cube_parts::face::{test_face, Face};
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub struct Cube {
    pub white: Face,
    pub red: Face,
    pub green: Face,
    pub blue: Face,
    pub yellow: Face,
    pub orange: Face,
}

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let spacer = "        ";
        write!(
            f,
            "{}{}\n{}{}\n{}{}\n{} {} {} {}\n{} {} {} {}\n{} {} {} {}\n{}{}\n{}{}\n{}{}",
            spacer,
            self.orange.t,
            spacer,
            self.orange.m,
            spacer,
            self.orange.b,
            self.green.t,
            self.white.t,
            self.blue.t,
            self.yellow.t,
            self.green.m,
            self.white.m,
            self.blue.m,
            self.yellow.m,
            self.green.b,
            self.white.b,
            self.blue.b,
            self.yellow.b,
            spacer,
            self.red.t,
            spacer,
            self.red.m,
            spacer,
            self.red.b
        )
    }
}

pub fn build_cube(args: &Vec<String>) -> Cube {
    let white = build_side(&args, 1);
    let orange = build_side(&args, 10);
    let yellow = build_side(&args, 19);
    let red = build_side(&args, 28);
    let green = build_side(&args, 37);
    let blue = build_side(&args, 46);
    Cube {
        white,
        orange,
        yellow,
        red,
        green,
        blue,
    }
}

pub fn test_cube(cube: &Cube) -> () {
    let mut counts = Counts {
        y: 0,
        g: 0,
        r: 0,
        w: 0,
        o: 0,
        b: 0,
    };

    counts = test_face(&cube.yellow, counts, 'y');
    counts = test_face(&cube.green, counts, 'g');
    counts = test_face(&cube.red, counts, 'r');
    counts = test_face(&cube.white, counts, 'w');
    counts = test_face(&cube.orange, counts, 'o');
    counts = test_face(&cube.blue, counts, 'b');

    assert_eq!(counts.g, 9, "Green");
    assert_eq!(counts.r, 9, "Red");
    assert_eq!(counts.w, 9, "White");
    assert_eq!(counts.o, 9, "Orange");
    assert_eq!(counts.b, 9, "Blue");
    assert_eq!(counts.y, 9, "Yellow");
}

pub struct Counts {
    pub y: u8,
    pub g: u8,
    pub r: u8,
    pub w: u8,
    pub o: u8,
    pub b: u8,
}
