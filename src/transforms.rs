use crate::cube_parts::cube::Cube;
use crate::cube_parts::face::{rotate_face_clockwise, rotate_face_counter_clockwise, Orientation};

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
    working_cube.white = rotate_face_clockwise(working_cube.white);
    let temp_top_edge = working_cube.orange.clone();
    let temp_right_edge = working_cube.blue.clone();
    let temp_down_edge = working_cube.red.clone();
    let temp_left_edge = working_cube.green.clone();
    working_cube.orange.b.l = temp_left_edge.b.r;
    working_cube.orange.b.c = temp_left_edge.m.r;
    working_cube.orange.b.r = temp_left_edge.t.r;
    working_cube.blue.t.l = temp_top_edge.b.l;
    working_cube.blue.m.l = temp_top_edge.b.c;
    working_cube.blue.b.l = temp_top_edge.b.r;
    working_cube.red.t.l = temp_right_edge.t.l;
    working_cube.red.t.c = temp_right_edge.m.l;
    working_cube.red.t.r = temp_right_edge.b.l;
    working_cube.green.t.r = temp_down_edge.t.l;
    working_cube.green.m.r = temp_down_edge.t.c;
    working_cube.green.b.r = temp_down_edge.t.r;
    working_cube
}

fn fp(mut working_cube: Cube) -> Cube {
    working_cube.white = rotate_face_counter_clockwise(working_cube.white);
    let temp_top_edge = working_cube.orange.clone();
    let temp_right_edge = working_cube.blue.clone();
    let temp_down_edge = working_cube.red.clone();
    let temp_left_edge = working_cube.green.clone();
    working_cube.orange.b.l = temp_right_edge.t.l;
    working_cube.orange.b.c = temp_right_edge.m.l;
    working_cube.orange.b.r = temp_right_edge.b.l;
    working_cube.blue.t.l = temp_down_edge.t.l;
    working_cube.blue.m.l = temp_down_edge.t.c;
    working_cube.blue.b.l = temp_down_edge.t.r;
    working_cube.red.t.l = temp_left_edge.t.r;
    working_cube.red.t.c = temp_left_edge.m.r;
    working_cube.red.t.r = temp_left_edge.b.r;
    working_cube.green.t.r = temp_top_edge.b.l;
    working_cube.green.m.r = temp_top_edge.b.c;
    working_cube.green.b.r = temp_top_edge.b.r;
    working_cube
}

fn r(mut working_cube: Cube) -> Cube {
    working_cube.blue = rotate_face_clockwise(working_cube.blue);
    let temp_top_edge = working_cube.orange.clone();
    let temp_right_edge = working_cube.yellow.clone();
    let temp_down_edge = working_cube.red.clone();
    let temp_left_edge = working_cube.white.clone();
    working_cube.orange.t.r = temp_left_edge.t.r;
    working_cube.orange.m.r = temp_left_edge.m.r;
    working_cube.orange.b.r = temp_left_edge.b.r;
    working_cube.yellow.t.l = temp_top_edge.b.r;
    working_cube.yellow.m.l = temp_top_edge.m.r;
    working_cube.yellow.b.l = temp_top_edge.t.r;
    working_cube.red.t.r = temp_right_edge.b.l;
    working_cube.red.m.r = temp_right_edge.m.l;
    working_cube.red.b.r = temp_right_edge.t.l;
    working_cube.white.t.r = temp_down_edge.t.r;
    working_cube.white.m.r = temp_down_edge.m.r;
    working_cube.white.b.r = temp_down_edge.b.r;
    working_cube
}

fn rp(mut working_cube: Cube) -> Cube {
    working_cube.blue = rotate_face_counter_clockwise(working_cube.blue);
    let temp_top_edge = working_cube.orange.clone();
    let temp_right_edge = working_cube.yellow.clone();
    let temp_down_edge = working_cube.red.clone();
    let temp_left_edge = working_cube.white.clone();
    working_cube.orange.t.r = temp_right_edge.t.l;
    working_cube.orange.m.r = temp_right_edge.m.l;
    working_cube.orange.b.r = temp_right_edge.b.l;
    working_cube.yellow.t.l = temp_down_edge.b.r;
    working_cube.yellow.m.l = temp_down_edge.m.r;
    working_cube.yellow.b.l = temp_down_edge.t.r;
    working_cube.red.t.r = temp_left_edge.t.r;
    working_cube.red.m.r = temp_left_edge.m.r;
    working_cube.red.b.r = temp_left_edge.b.r;
    working_cube.white.t.r = temp_top_edge.b.r;
    working_cube.white.m.r = temp_top_edge.m.r;
    working_cube.white.b.r = temp_top_edge.t.r;
    working_cube
}

fn l(mut working_cube: Cube) -> Cube {
    working_cube.green = rotate_face_clockwise(working_cube.green);
    let temp_top_edge = working_cube.orange.clone();
    let temp_right_edge = working_cube.white.clone();
    let temp_down_edge = working_cube.red.clone();
    let temp_left_edge = working_cube.yellow.clone();
    working_cube.orange.t.l = temp_left_edge.b.r;
    working_cube.orange.m.l = temp_left_edge.m.r;
    working_cube.orange.b.l = temp_left_edge.t.r;
    working_cube.white.t.l = temp_top_edge.t.l;
    working_cube.white.m.l = temp_top_edge.m.l;
    working_cube.white.b.l = temp_top_edge.b.l;
    working_cube.red.t.l = temp_right_edge.t.l;
    working_cube.red.m.l = temp_right_edge.m.l;
    working_cube.red.b.l = temp_right_edge.b.l;
    working_cube.yellow.t.r = temp_down_edge.t.l;
    working_cube.yellow.m.r = temp_down_edge.m.l;
    working_cube.yellow.b.r = temp_down_edge.b.l;
    working_cube
}

fn lp(mut working_cube: Cube) -> Cube {
    working_cube.green = rotate_face_counter_clockwise(working_cube.green);
    let temp_top_edge = working_cube.orange.clone();
    let temp_right_edge = working_cube.white.clone();
    let temp_down_edge = working_cube.red.clone();
    let temp_left_edge = working_cube.yellow.clone();
    working_cube.orange.t.l = temp_right_edge.b.l;
    working_cube.orange.m.l = temp_right_edge.m.l;
    working_cube.orange.b.l = temp_right_edge.t.l;
    working_cube.white.t.l = temp_down_edge.b.l;
    working_cube.white.m.l = temp_down_edge.m.l;
    working_cube.white.b.l = temp_down_edge.t.l;
    working_cube.red.t.l = temp_left_edge.b.r;
    working_cube.red.m.l = temp_left_edge.m.r;
    working_cube.red.b.l = temp_left_edge.t.r;
    working_cube.yellow.t.r = temp_top_edge.b.l;
    working_cube.yellow.m.r = temp_top_edge.m.l;
    working_cube.yellow.b.r = temp_top_edge.t.l;
    working_cube
}

fn u(mut working_cube: Cube) -> Cube {
    working_cube.orange = rotate_face_clockwise(working_cube.orange);
    let temp_top_edge = working_cube.yellow.clone();
    let temp_right_edge = working_cube.blue.clone();
    let temp_down_edge = working_cube.white.clone();
    let temp_left_edge = working_cube.green.clone();
    working_cube.yellow.t.l = temp_left_edge.t.r;
    working_cube.yellow.t.c = temp_left_edge.t.c;
    working_cube.yellow.t.r = temp_left_edge.t.l;
    working_cube.blue.t.l = temp_top_edge.t.r;
    working_cube.blue.t.c = temp_top_edge.t.c;
    working_cube.blue.t.r = temp_top_edge.t.l;
    working_cube.white.t.l = temp_right_edge.t.r;
    working_cube.white.t.c = temp_right_edge.t.c;
    working_cube.white.t.r = temp_right_edge.t.l;
    working_cube.green.t.r = temp_down_edge.t.l;
    working_cube.green.t.c = temp_down_edge.t.c;
    working_cube.green.t.l = temp_down_edge.t.r;
    working_cube
}

fn up(mut working_cube: Cube) -> Cube {
    working_cube.orange = rotate_face_counter_clockwise(working_cube.orange);
    let temp_top_edge = working_cube.yellow.clone();
    let temp_right_edge = working_cube.blue.clone();
    let temp_down_edge = working_cube.white.clone();
    let temp_left_edge = working_cube.green.clone();
    working_cube.yellow.t.l = temp_right_edge.t.r;
    working_cube.yellow.t.c = temp_right_edge.t.c;
    working_cube.yellow.t.r = temp_right_edge.t.l;
    working_cube.blue.t.l = temp_down_edge.t.r;
    working_cube.blue.t.c = temp_down_edge.t.c;
    working_cube.blue.t.r = temp_down_edge.t.l;
    working_cube.white.t.l = temp_left_edge.t.r;
    working_cube.white.t.c = temp_left_edge.t.c;
    working_cube.white.t.r = temp_left_edge.t.l;
    working_cube.green.t.r = temp_top_edge.t.l;
    working_cube.green.t.c = temp_top_edge.t.c;
    working_cube.green.t.l = temp_top_edge.t.r;
    working_cube
}

fn d(mut working_cube: Cube) -> Cube {
    working_cube.red = rotate_face_counter_clockwise(working_cube.red);
    let temp_white_face = working_cube.white.clone();
    let temp_blue_face = working_cube.blue.clone();
    let temp_yellow_face = working_cube.yellow.clone();
    let temp_green_face = working_cube.green.clone();
    working_cube.yellow.b.l = temp_green_face.b.r;
    working_cube.yellow.b.c = temp_green_face.b.c;
    working_cube.yellow.b.r = temp_green_face.b.l;
    working_cube.blue.b.l = temp_yellow_face.b.r;
    working_cube.blue.b.c = temp_yellow_face.b.c;
    working_cube.blue.b.r = temp_yellow_face.b.l;
    working_cube.white.b.l = temp_blue_face.b.l;
    working_cube.white.b.c = temp_blue_face.b.c;
    working_cube.white.b.r = temp_blue_face.b.r;
    working_cube.green.b.l = temp_white_face.b.l;
    working_cube.green.b.c = temp_white_face.b.c;
    working_cube.green.b.r = temp_white_face.b.r;
    working_cube
}

fn dp(mut working_cube: Cube) -> Cube {
    working_cube.red = rotate_face_clockwise(working_cube.red);
    let temp_yellow_face = working_cube.yellow.clone();
    let temp_blue_face = working_cube.blue.clone();
    let temp_white_face = working_cube.white.clone();
    let temp_green_face = working_cube.green.clone();
    working_cube.yellow.b.l = temp_blue_face.b.r;
    working_cube.yellow.b.c = temp_blue_face.b.c;
    working_cube.yellow.b.r = temp_blue_face.b.l;
    working_cube.blue.b.l = temp_white_face.b.l;
    working_cube.blue.b.c = temp_white_face.b.c;
    working_cube.blue.b.r = temp_white_face.b.r;
    working_cube.white.b.l = temp_green_face.b.l;
    working_cube.white.b.c = temp_green_face.b.c;
    working_cube.white.b.r = temp_green_face.b.r;
    working_cube.green.b.r = temp_yellow_face.b.l;
    working_cube.green.b.c = temp_yellow_face.b.c;
    working_cube.green.b.l = temp_yellow_face.b.r;
    working_cube
}

fn b(mut working_cube: Cube) -> Cube {
    working_cube.yellow = rotate_face_counter_clockwise(working_cube.yellow);
    let temp_top_edge = working_cube.orange.clone();
    let temp_right_edge = working_cube.green.clone();
    let temp_down_edge = working_cube.red.clone();
    let temp_left_edge = working_cube.blue.clone();
    working_cube.orange.t.l = temp_right_edge.b.l;
    working_cube.orange.t.c = temp_right_edge.m.l;
    working_cube.orange.t.r = temp_right_edge.t.l;
    working_cube.green.t.l = temp_down_edge.b.l;
    working_cube.green.m.l = temp_down_edge.b.c;
    working_cube.green.b.l = temp_down_edge.b.r;
    working_cube.red.b.l = temp_left_edge.b.r;
    working_cube.red.b.c = temp_left_edge.m.r;
    working_cube.red.b.r = temp_left_edge.t.r;
    working_cube.blue.t.r = temp_top_edge.t.l;
    working_cube.blue.m.r = temp_top_edge.t.c;
    working_cube.blue.b.r = temp_top_edge.t.r;
    working_cube
}

fn bp(mut working_cube: Cube) -> Cube {
    working_cube.yellow = rotate_face_clockwise(working_cube.yellow);
    let temp_orange_face = working_cube.orange.clone();
    let temp_green_face = working_cube.green.clone();
    let temp_red_face = working_cube.red.clone();
    let temp_blue_face = working_cube.blue.clone();
    working_cube.orange.t.l = temp_blue_face.t.r;
    working_cube.orange.t.c = temp_blue_face.m.r;
    working_cube.orange.t.r = temp_blue_face.b.r;
    working_cube.green.t.l = temp_orange_face.t.r;
    working_cube.green.m.l = temp_orange_face.t.c;
    working_cube.green.b.l = temp_orange_face.t.l;
    working_cube.red.b.l = temp_green_face.t.l;
    working_cube.red.b.c = temp_green_face.m.l;
    working_cube.red.b.r = temp_green_face.b.l;
    working_cube.blue.t.r = temp_red_face.b.r;
    working_cube.blue.m.r = temp_red_face.b.c;
    working_cube.blue.b.r = temp_red_face.b.l;
    working_cube
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
