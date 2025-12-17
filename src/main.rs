use crate::cube::*;

mod cube;

fn main() {
    let mut cube = Cube::solved();
    cube.rotate(Rotate::U);
    cube.print();
    cube.rotate(Rotate::B);
    cube.print();
    cube.rotate(Rotate::RB);
    cube.print();
}
