use crate::cube::*;

mod cube;
mod ui;

// double rotates dont matter
// single rotates swap the move direction

fn main() {
    // let _ = ui::main();

    let solved = Cube::solved();
    let mut scrambled = Cube::solved();
    let moves: Vec<Rotate> = (0..5)
        .map(|_| Rotate::from_num(rand::random_range(0..35)))
        .collect();
    scrambled.apply_rotations(&moves);
    scrambled.print_normalized();
    println!("scrambled using: {:?}", moves);

    let steps = scrambled.get_steps(&solved);
    scrambled.apply_rotations(&steps);
    scrambled.print_normalized();

    println!("{:?}", steps);
}
