use crate::cube::*;

mod cube;

fn rubiks_maneuver(cube: &mut Cube) {
    cube.apply_rotations(&[
        Rotate::RR,
        Rotate::L,
        Rotate::RF,
        Rotate::RR,
        Rotate::L,
        Rotate::RD,
        Rotate::RR,
        Rotate::L,
        Rotate::B,
        Rotate::B,
        Rotate::R,
        Rotate::RL,
        Rotate::RD,
        Rotate::R,
        Rotate::RL,
        Rotate::RF,
        Rotate::R,
        Rotate::RL,
        Rotate::U,
        Rotate::U,
    ]);
}

fn main() {
    let mut cube = Cube::solved();
    cube.print();
    rubiks_maneuver(&mut cube);
    cube.print();
}
