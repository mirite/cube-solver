use crate::cube_parts::cube::Cube;

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
    let temp_cube = working_cube.clone();
    working_cube.orange.b.l = temp_cube.green.b.l;
    working_cube.orange.b.c = temp_cube.green.b.c;
    working_cube.orange.b.r = temp_cube.green.b.r;
    working_cube.blue.t.l = temp_cube.orange.b.l;
    working_cube.blue.m.l = temp_cube.orange.b.c;
    working_cube.blue.b.l = temp_cube.orange.b.r;
    working_cube.red.t.l = temp_cube.blue.b.l;
    working_cube.red.t.c = temp_cube.blue.b.c;
    working_cube.red.t.r = temp_cube.blue.b.r;
    working_cube.green.t.r = temp_cube.red.b.l;
    working_cube.green.m.r = temp_cube.red.b.c;
    working_cube.green.b.r = temp_cube.red.b.r;
    working_cube.white.t.l = temp_cube.white.b.l;
    working_cube.white.t.c = temp_cube.white.m.c;
    working_cube.white.t.r = temp_cube.white.t.l;
    working_cube.white.m.l = temp_cube.white.b.c;
    working_cube.white.m.r = temp_cube.white.t.c;
    working_cube.white.b.l = temp_cube.white.b.r;
    working_cube.white.b.c = temp_cube.white.m.r;
    working_cube.white.b.r = temp_cube.white.t.r;
    working_cube
}

fn fp(old_cube: Cube) -> Cube {
    let temp_cube = old_cube.clone();
    old_cube
}

fn r(old_cube: Cube) -> Cube {
    let temp_cube = old_cube.clone();
    old_cube
}

fn rp(old_cube: Cube) -> Cube {
    let temp_cube = old_cube.clone();
    old_cube
}

fn l(old_cube: Cube) -> Cube {
    let temp_cube = old_cube.clone();
    old_cube
}

fn lp(old_cube: Cube) -> Cube {
    let temp_cube = old_cube.clone();
    old_cube
}

fn u(old_cube: Cube) -> Cube {
    let temp_cube = old_cube.clone();
    old_cube
}

fn up(old_cube: Cube) -> Cube {
    let temp_cube = old_cube.clone();
    old_cube
}

fn d(old_cube: Cube) -> Cube {
    let temp_cube = old_cube.clone();
    old_cube
}

fn dp(old_cube: Cube) -> Cube {
    let temp_cube = old_cube.clone();
    old_cube
}

fn b(old_cube: Cube) -> Cube {
    let temp_cube = old_cube.clone();
    old_cube
}

fn bp(old_cube: Cube) -> Cube {
    let temp_cube = old_cube.clone();
    old_cube
}

pub fn turn_cube(turn: Turns, cube: Cube) -> Cube {
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
