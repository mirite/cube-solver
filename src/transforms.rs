use crate::cube_parts::cube::Cube;
use crate::cube_parts::face::{rotate_clockwise, rotate_counter_clockwise};

#[derive(Eq, Hash, PartialEq)]
pub enum Turns {
    F,
    Fp,
    R,
    Rp,
    L,
    Lp,
    U,
    Up,
    D,
    Dp,
    B,
    Bp,
}

fn f(mut working_cube: Cube) -> Cube {
    rotate_clockwise(
        &mut working_cube.white,
        &mut working_cube.orange,
        &mut working_cube.blue,
        &mut working_cube.red,
        &mut working_cube.green,
    );
    working_cube
}

fn fp(mut working_cube: Cube) -> Cube {
    rotate_counter_clockwise(
        &mut working_cube.white,
        &mut working_cube.orange,
        &mut working_cube.blue,
        &mut working_cube.red,
        &mut working_cube.green,
    );
    working_cube
}

fn r(mut working_cube: Cube) -> Cube {
    working_cube
}

fn rp(mut working_cube: Cube) -> Cube {
    working_cube
}

fn l(mut working_cube: Cube) -> Cube {
    working_cube
}

fn lp(mut working_cube: Cube) -> Cube {
    working_cube
}

fn u(mut working_cube: Cube) -> Cube {
    working_cube
}

fn up(mut working_cube: Cube) -> Cube {
    working_cube
}

fn d(mut working_cube: Cube) -> Cube {
    working_cube
}

fn dp(mut working_cube: Cube) -> Cube {
    working_cube
}

fn b(mut working_cube: Cube) -> Cube {
    working_cube
}

fn bp(mut working_cube: Cube) -> Cube {
    working_cube
}

pub fn turn_cube(turn: Turns, mut cube: Cube) -> Cube {
    match turn {
        Turns::F => f(cube),
        Turns::Fp => fp(cube),
        Turns::R => r(cube),
        Turns::Rp => rp(cube),
        Turns::L => l(cube),
        Turns::Lp => lp(cube),
        Turns::U => u(cube),
        Turns::Up => up(cube),
        Turns::D => d(cube),
        Turns::Dp => dp(cube),
        Turns::B => b(cube),
        Turns::Bp => bp(cube),
    }
}
