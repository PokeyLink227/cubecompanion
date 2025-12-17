static ASCIICUBE: [[char; 25]; 17] = [
    [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '+', '-', '-', '-', '-', '-', '-', '-', '+', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ],
    [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', '.', ' ', '.', ' ', '.', ' ', '|', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ],
    [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', '.', ' ', 'R', ' ', '.', ' ', '|', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ],
    [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', '.', ' ', '.', ' ', '.', ' ', '|', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ],
    [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '+', '-', '-', '-', '-', '-', '-', '-', '+', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ],
    [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', '.', ' ', '.', ' ', '.', ' ', '|', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ],
    [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', '.', ' ', 'W', ' ', '.', ' ', '|', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ],
    [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', '.', ' ', '.', ' ', '.', ' ', '|', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ],
    [
        '+', '-', '-', '-', '-', '-', '-', '-', '+', '-', '-', '-', '-', '-', '-', '-', '+', '-',
        '-', '-', '-', '-', '-', '-', '+',
    ],
    [
        '|', ' ', '.', ' ', '.', ' ', '.', ' ', '|', ' ', '.', ' ', '.', ' ', '.', ' ', '|', ' ',
        '.', ' ', '.', ' ', '.', ' ', '|',
    ],
    [
        '|', ' ', '.', ' ', 'B', ' ', '.', ' ', '|', ' ', '.', ' ', 'O', ' ', '.', ' ', '|', ' ',
        '.', ' ', 'G', ' ', '.', ' ', '|',
    ],
    [
        '|', ' ', '.', ' ', '.', ' ', '.', ' ', '|', ' ', '.', ' ', '.', ' ', '.', ' ', '|', ' ',
        '.', ' ', '.', ' ', '.', ' ', '|',
    ],
    [
        '+', '-', '-', '-', '-', '-', '-', '-', '+', '-', '-', '-', '-', '-', '-', '-', '+', '-',
        '-', '-', '-', '-', '-', '-', '+',
    ],
    [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', '.', ' ', '.', ' ', '.', ' ', '|', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ],
    [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', '.', ' ', 'Y', ' ', '.', ' ', '|', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ],
    [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', ' ', '.', ' ', '.', ' ', '.', ' ', '|', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ],
    [
        ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '+', '-', '-', '-', '-', '-', '-', '-', '+', ' ',
        ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ],
];

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White = 1,
    Orange = 2,
    Green = 3,
    Yellow = 4,
    Red = 5,
    Blue = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotate {
    // standard
    U,
    D,
    R,
    L,
    F,
    B,
    Up,
    Dp,
    Rp,
    Lp,
    Fp,
    Bp,

    // movement
    x,
    y,
    z,
    xp,
    yp,
    zp,

    // middle
    M,
    Mp,
}

impl Rotate {
    pub fn prime(self) -> Self {
        match self {
            Self::U => Self::Up,
            Self::D => Self::Dp,
            Self::R => Self::Rp,
            Self::L => Self::Lp,
            Self::F => Self::Fp,
            Self::B => Self::Bp,
            Self::Up => Self::U,
            Self::Dp => Self::D,
            Self::Rp => Self::R,
            Self::Lp => Self::L,
            Self::Fp => Self::F,
            Self::Bp => Self::B,
            Self::x => Self::xp,
            Self::y => Self::yp,
            Self::z => Self::zp,
            Self::xp => Self::x,
            Self::yp => Self::y,
            Self::zp => Self::z,
            Self::M => Self::Mp,
            Self::Mp => Self::M,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edge {
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}

impl Edge {
    pub const fn rotate(self, amt: i8) -> Self {
        let new = (self as i8 + amt) as u8 & 0b11;
        match new {
            0 => Self::Top,
            1 => Self::Right,
            2 => Self::Bottom,
            3 => Self::Left,
            _ => unreachable!(),
        }
    }
}

impl Color {
    pub fn char(self) -> char {
        match self {
            Self::White => 'W',
            Self::Orange => 'O',
            Self::Green => 'G',
            Self::Yellow => 'Y',
            Self::Red => 'R',
            Self::Blue => 'B',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Face(u32);

impl Face {
    pub fn from_array(colors: &[Color; 8]) -> Self {
        let mut buf: u32 = 0;
        for &c in colors {
            buf = (buf << 3) | (c as u32 & 0b0111);
        }
        Face(buf)
    }

    fn rotate_mut(&mut self, dir: Direction) {
        match dir {
            Direction::Clockwise => {
                *self = Face(((self.0 >> 6) & 0b111111111111111111) | ((self.0 & 0b111111) << 18));
            }
            Direction::CounterClockwise => {
                *self = Face((self.0 << 6) | (self.0 >> 18));
            }
        }
    }

    // const fn reverse_edge(old: u32) -> u32 {
    //     (old >> 6 & 0b000_000_111) | (old & 0b000_111_000) | (old << 6 & 0b111_000_000)
    // }

    const fn get_edge(&self, edge: Edge) -> u32 {
        match edge {
            Edge::Top => (self.0 >> 15) & 0b111_111_111,
            Edge::Right => (self.0 >> 9) & 0b111_111_111,
            Edge::Bottom => (self.0 >> 3) & 0b111_111_111,
            Edge::Left => ((self.0 << 3) | (self.0 >> 21)) & 0b111_111_111,
        }
    }

    const fn set_edge_mut(&mut self, edge: Edge, new: u32) -> u32 {
        match edge {
            Edge::Top => {
                let old = (self.0 >> 15) & 0b111_111_111;
                self.0 = (self.0 & !(0b111_111_111 << 15)) | (new << 15);
                self.0 &= 0b111_111_111_111_111_111_111_111;
                old
            }
            Edge::Bottom => {
                let temp = (self.0 >> 3) & 0b111_111_111;
                self.0 = (self.0 & !(0b111_111_111 << 3)) | (new << 3);
                self.0 &= 0b111_111_111_111_111_111_111_111;
                temp
            }
            Edge::Right => {
                let old = (self.0 >> 9) & 0b111_111_111;
                self.0 = (self.0 & !(0b111_111_111 << 9)) | (new << 9);
                self.0 &= 0b111_111_111_111_111_111_111_111;
                old
            }
            Edge::Left => {
                let old = ((self.0 << 3) | (self.0 >> 21)) & 0b111_111_111;
                self.0 = self.0 & !(0b111 << 21) & !(0b111_111);
                self.0 = self.0 | (new >> 3) | ((new & 0b111) << 21);
                self.0 &= 0b111_111_111_111_111_111_111_111;
                old
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cube {
    faces: [Face; 6],
    mapping: [(usize, i8); 6],
}

impl Cube {
    pub fn solved() -> Self {
        Self {
            faces: [
                Face::from_array(&[Color::Red; 8]),
                Face::from_array(&[Color::White; 8]),
                Face::from_array(&[Color::Blue; 8]),
                Face::from_array(&[Color::Orange; 8]),
                Face::from_array(&[Color::Green; 8]),
                Face::from_array(&[Color::Yellow; 8]),
            ],
            mapping: [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
        }
    }

    pub fn rotate(&mut self, mut action: Rotate) {
        use Edge as E;

        println!("{action:?}");

        // let mapping = [0, 1, 2, 3, 4, 5];
        // let mapping = [1, 3, 2, 5, 4, 0];
        // let mapping = [5, 0, 2, 1, 4, 3];

        let s1: (usize, Direction);
        let s2: [(usize, E); 4];

        match action {
            Rotate::x => {
                let temp = self.mapping[0];
                self.mapping[0] = self.mapping[1];
                self.mapping[1] = self.mapping[3];
                self.mapping[3] = self.mapping[5];
                self.mapping[5] = temp;
                self.mapping[2].1 += 1;
                self.mapping[4].1 -= 1;
                return;
            }
            Rotate::xp => {
                let temp = self.mapping[5];
                self.mapping[5] = self.mapping[3];
                self.mapping[3] = self.mapping[1];
                self.mapping[1] = self.mapping[0];
                self.mapping[0] = temp;
                self.mapping[2].1 -= 1;
                self.mapping[4].1 += 1;
                return;
            }
            Rotate::y => {
                let temp = self.mapping[0];
                self.mapping[0] = self.mapping[1];
                self.mapping[1] = self.mapping[3];
                self.mapping[3] = self.mapping[5];
                self.mapping[5] = temp;
                return;
            }
            Rotate::z => {
                let temp = self.mapping[0];
                self.mapping[0] = self.mapping[1];
                self.mapping[1] = self.mapping[3];
                self.mapping[3] = self.mapping[5];
                self.mapping[5] = temp;
                return;
            }
            Rotate::M => {
                self.apply_rotations(&[Rotate::R, Rotate::Lp, Rotate::xp]);
                return;
            }
            Rotate::Mp => {
                self.apply_rotations(&[Rotate::Rp, Rotate::L, Rotate::x]);
                return;
            }
            Rotate::U => {
                s1 = (1, Direction::Clockwise);
                s2 = [(0, E::Bottom), (4, E::Top), (3, E::Top), (2, E::Top)];
            }
            Rotate::Up => {
                s1 = (1, Direction::CounterClockwise);
                s2 = [(0, E::Bottom), (2, E::Top), (3, E::Top), (4, E::Top)];
            }
            Rotate::D => {
                s1 = (5, Direction::Clockwise);
                s2 = [(2, E::Bottom), (3, E::Bottom), (4, E::Bottom), (0, E::Top)];
            }
            Rotate::Dp => {
                s1 = (5, Direction::CounterClockwise);
                s2 = [(2, E::Bottom), (0, E::Top), (4, E::Bottom), (3, E::Bottom)];
            }
            Rotate::R => {
                s1 = (4, Direction::Clockwise);
                s2 = [(5, E::Right), (3, E::Right), (1, E::Right), (0, E::Right)];
            }
            Rotate::Rp => {
                s1 = (4, Direction::CounterClockwise);
                s2 = [(5, E::Right), (0, E::Right), (1, E::Right), (3, E::Right)];
            }
            Rotate::L => {
                s1 = (2, Direction::Clockwise);
                s2 = [(5, E::Left), (0, E::Left), (1, E::Left), (3, E::Left)];
            }
            Rotate::Lp => {
                s1 = (2, Direction::CounterClockwise);
                s2 = [(5, E::Left), (3, E::Left), (1, E::Left), (0, E::Left)];
            }
            Rotate::F => {
                s1 = (3, Direction::Clockwise);
                s2 = [(1, E::Bottom), (4, E::Left), (5, E::Top), (2, E::Right)];
            }
            Rotate::Fp => {
                s1 = (3, Direction::CounterClockwise);
                s2 = [(1, E::Bottom), (2, E::Right), (5, E::Top), (4, E::Left)];
            }
            Rotate::B => {
                s1 = (0, Direction::Clockwise);
                s2 = [(1, E::Top), (2, E::Left), (5, E::Bottom), (4, E::Right)];
            }
            Rotate::Bp => {
                s1 = (3, Direction::CounterClockwise);
                s2 = [(1, E::Top), (4, E::Right), (5, E::Bottom), (2, E::Left)];
            }
            _ => todo!(),
        }

        // Step 1 rotate pieces on face
        self.faces[self.mapping[s1.0].0].rotate_mut(s1.1);

        // Step 2 move pieces between faces
        let a =
            self.faces[self.mapping[s2[0].0].0].get_edge(s2[0].1.rotate(self.mapping[s2[0].0].1));
        let b = self.faces[self.mapping[s2[1].0].0]
            .set_edge_mut(s2[1].1.rotate(self.mapping[s2[1].0].1), a);
        let c = self.faces[self.mapping[s2[2].0].0]
            .set_edge_mut(s2[2].1.rotate(self.mapping[s2[2].0].1), b);
        let d = self.faces[self.mapping[s2[3].0].0]
            .set_edge_mut(s2[3].1.rotate(self.mapping[s2[3].0].1), c);
        self.faces[self.mapping[s2[0].0].0]
            .set_edge_mut(s2[0].1.rotate(self.mapping[s2[0].0].1), d);
    }

    pub fn apply_rotations(&mut self, actions: &[Rotate]) {
        for &a in actions {
            self.rotate(a);
        }
    }

    pub fn print(&self) {
        let offsets = [(10, 1), (10, 5), (2, 9), (10, 9), (18, 9), (10, 13)];
        let sq_off = [
            (0, 0),
            (2, 0),
            (4, 0),
            (4, 1),
            (4, 2),
            (2, 2),
            (0, 2),
            (0, 1),
        ];
        let colors = ['0', 'W', 'O', 'G', 'Y', 'R', 'B', '7'];
        let mut ac: [[char; 25]; 17] = ASCIICUBE.clone();

        // populate the proper colors
        for (i, f) in self.faces.iter().enumerate() {
            for sq in 0..8 {
                let x = offsets[i].0 + sq_off[sq].0;
                let y = offsets[i].1 + sq_off[sq].1;
                let c = ((f.0 >> ((7 - sq) * 3)) & 0b0111) as usize;
                ac[y][x] = colors[c];
            }
        }

        // print the cube
        for line in ac {
            for ch in line {
                print!("{ch}");
            }
            println!();
        }
    }

    // pub fn get_face(&self, face: Color) -> &Face {
    //     &self.faces[(face as usize * 3)..((face as usize + 1) * 3)]
    // }
}

#[cfg(test)]
mod tests {
    use crate::cube::*;

    #[test]
    fn t1() {
        let mut cube = Cube::solved();
        cube.rotate(Rotate::R);
        cube.rotate(Rotate::x);
        cube.rotate(Rotate::U);
        cube.print();

        let mut cube2 = Cube::solved();
        cube2.rotate(Rotate::R);
        cube2.rotate(Rotate::F);
        cube2.print();

        assert_eq!(cube.faces, cube2.faces);
    }

    #[test]
    fn t2() {
        let mut cube = Cube::solved();
        cube.rotate(Rotate::Mp);
        cube.rotate(Rotate::U);
        cube.rotate(Rotate::Mp);
        cube.print();

        let mut cube2 = Cube::solved();
        cube2.rotate(Rotate::Rp);
        cube2.rotate(Rotate::L);
        cube2.rotate(Rotate::F);
        cube2.rotate(Rotate::Rp);
        cube2.rotate(Rotate::L);
        cube2.print();

        assert_eq!(cube.faces, cube2.faces);
    }
}
