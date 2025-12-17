use crate::cube::*;

mod cube;

fn main() {
    let mut cube = Cube::solved();
    cube.rotate(Rotate::R);
    cube.print();
    cube.rotate(Rotate::U);
    cube.print();
    cube.rotate(Rotate::RR);
    cube.print();
    cube.rotate(Rotate::D);
    cube.print();
}
