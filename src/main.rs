use std::env;
use std::fmt::{Display, Formatter};

struct Face {
    t: Row,
    m: Row,
    b: Row,
}

struct Cube {
    white: Face,
    red: Face,
    green: Face,
    blue: Face,
    yellow: Face,
    orange: Face,
}

struct Row {
    l: char,
    c: char,
    r: char,
}

impl Display for Face {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, " _ _ _ \n{}\n{}\n{}\n - - - ", self.t, self.m, self.b)
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "|{}|{}|{}|", self.l, self.c, self.r)
    }
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

fn parse_square(args: &Vec<String>, index: usize) -> char {
    let chars: Vec<_> = args[index].chars().collect();
    chars[0]
}

fn build_side(args: &Vec<String>, start: usize) -> Face {
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

fn build_cube(args: &Vec<String>) -> Cube {
    Cube {
        white: build_side(&args, 1),
        orange: build_side(&args, 10),
        yellow: build_side(&args, 19),
        red: build_side(&args, 28),
        green: build_side(&args, 37),
        blue: build_side(&args, 46),
    }
}

fn test_cube(cube: &Cube) -> () {
    assert_eq!(cube.yellow.m.c, 'y');
    assert_eq!(cube.green.m.c, 'g');
    assert_eq!(cube.red.m.c, 'r');
    assert_eq!(cube.white.m.c, 'w');
    assert_eq!(cube.orange.m.c, 'o');
    assert_eq!(cube.blue.m.c, 'b');
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cube = build_cube(&args);
    test_cube(&cube);
    println!("{}", cube);
}
