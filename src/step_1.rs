use crate::cube_parts::cube::Cube;
use crate::transforms::Turns;

pub fn step_1(cube: &mut Cube, turns: &mut Vec<Turns>) -> () {}

pub fn is_solved(cube: &Cube) -> bool {
    cube.white.m.c == 'w'
        && cube.white.t.c == 'w'
        && cube.orange.b.c == 'o'
        && cube.white.m.r == 'w'
        && cube.blue.m.l == 'b'
        && cube.white.b.c == 'w'
        && cube.red.t.c == 'r'
        && cube.white.m.l == 'w'
        && cube.green.m.r == 'g'
}
