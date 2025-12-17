use crate::cube::*;

mod cube;

fn rubiks_maneuver(cube: &mut Cube) {
    cube.apply_rotations(&[
        Rotate::Rp,
        Rotate::L,
        Rotate::Fp,
        Rotate::Rp,
        Rotate::L,
        Rotate::Dp,
        Rotate::Rp,
        Rotate::L,
        Rotate::B,
        Rotate::B,
        Rotate::R,
        Rotate::Lp,
        Rotate::Dp,
        Rotate::R,
        Rotate::Lp,
        Rotate::Fp,
        Rotate::R,
        Rotate::Lp,
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
