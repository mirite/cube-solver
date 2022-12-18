use super::face::{build_side, Face};
use std::fmt::{Display, Formatter};

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
            self.red.t,
            spacer,
            self.red.m,
            spacer,
            self.red.b,
            self.blue.t,
            self.white.t,
            self.green.t,
            self.yellow.t,
            self.blue.m,
            self.white.m,
            self.green.m,
            self.yellow.m,
            self.blue.b,
            self.white.b,
            self.green.b,
            self.yellow.b,
            spacer,
            self.orange.t,
            spacer,
            self.orange.m,
            spacer,
            self.orange.b
        )
    }
}

pub fn build_cube(args: &Vec<String>) -> Cube {
    Cube {
        white: build_side(&args, 1),
        orange: build_side(&args, 10),
        yellow: build_side(&args, 19),
        red: build_side(&args, 28),
        green: build_side(&args, 37),
        blue: build_side(&args, 46),
    }
}

pub fn test_cube(cube: &Cube) -> () {
    assert_eq!(cube.yellow.m.c, 'y');
    assert_eq!(cube.green.m.c, 'g');
    assert_eq!(cube.red.m.c, 'r');
    assert_eq!(cube.white.m.c, 'w');
    assert_eq!(cube.orange.m.c, 'o');
    assert_eq!(cube.blue.m.c, 'b');
}
