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
    // cube.apply_rotations(&[
    //     Rotate::Mp,
    //     Rotate::Up,
    //     Rotate::Mp,
    //     Rotate::Up,
    //     Rotate::Mp,
    //     Rotate::Up,
    //     Rotate::Up,
    //     // second half
    //     Rotate::M,
    //     Rotate::Up,
    //     Rotate::M,
    //     Rotate::Up,
    //     Rotate::M,
    //     Rotate::Up,
    //     Rotate::Up,
    // ]);
}

// double rotates dont matter
// single rotates swap the move direction

fn main() {
    let mut cube = Cube::solved();
    cube.print();

    let mut cube = Cube::solved();
    cube.rotate(Rotate::R);
    cube.rotate(Rotate::x);
    cube.rotate(Rotate::U);
    cube.print();

    let mut cube2 = Cube::solved();
    cube2.rotate(Rotate::R);
    cube2.rotate(Rotate::Fp);
    cube2.print();
}
