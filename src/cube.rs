use std::collections::HashMap;

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
    None,
    // Face Turns
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

    // Cube Roatations
    x,
    y,
    z,
    xp,
    yp,
    zp,

    // Slice Moves
    M,
    Mp,
    E,
    Ep,
    S,
    Sp,

    // Wide Moves
    Uw,
    Uwp,
    Dw,
    Dwp,
    Rw,
    Rwp,
    Lw,
    Lwp,
    Fw,
    Fwp,
    Bw,
    Bwp,
}

impl Rotate {
    pub fn prime(self) -> Self {
        match self {
            Self::None => Self::None,
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
            Self::E => Self::Ep,
            Self::Ep => Self::E,
            Self::S => Self::Sp,
            Self::Sp => Self::S,
            Self::Uw => Self::Uwp,
            Self::Dw => Self::Dwp,
            Self::Rw => Self::Rwp,
            Self::Lw => Self::Lwp,
            Self::Fw => Self::Fwp,
            Self::Bw => Self::Bwp,
            Self::Uwp => Self::Uw,
            Self::Dwp => Self::Dw,
            Self::Rwp => Self::Rw,
            Self::Lwp => Self::Lw,
            Self::Fwp => Self::Fw,
            Self::Bwp => Self::Bw,
        }
    }

    pub fn from_num(n: u8) -> Self {
        match n {
            0 => Self::U,
            1 => Self::D,
            2 => Self::R,
            3 => Self::L,
            4 => Self::F,
            5 => Self::B,
            6 => Self::Up,
            7 => Self::Dp,
            8 => Self::Rp,
            9 => Self::Lp,
            10 => Self::Fp,
            11 => Self::Bp,
            12 => Self::x,
            13 => Self::y,
            14 => Self::z,
            15 => Self::xp,
            16 => Self::yp,
            17 => Self::zp,
            18 => Self::M,
            19 => Self::Mp,
            20 => Self::E,
            21 => Self::Ep,
            22 => Self::S,
            23 => Self::Sp,
            24 => Self::Uw,
            25 => Self::Uwp,
            26 => Self::Dw,
            27 => Self::Dwp,
            28 => Self::Rw,
            29 => Self::Rwp,
            30 => Self::Lw,
            31 => Self::Lwp,
            32 => Self::Fw,
            33 => Self::Fwp,
            34 => Self::Bw,
            35 => Self::Bwp,
            _ => Self::None,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Face(u32);

impl Face {
    pub fn from_array(colors: &[Color; 8]) -> Self {
        let mut buf: u32 = 0;
        for &c in colors {
            buf = (buf << 3) | (c as u32 & 0b0111);
        }
        Face(buf)
    }

    const fn rotated(mut self, mut amt: i8) -> Self {
        while amt > 0 {
            self.rotate_mut(Direction::Clockwise);
            amt -= 1;
        }
        while amt < 0 {
            self.rotate_mut(Direction::CounterClockwise);
            amt += 1;
        }

        self
    }

    const fn rotate_mut(&mut self, dir: Direction) {
        match dir {
            Direction::Clockwise => {
                let temp = ((self.0 >> 6) & 0b111111111111111111) | ((self.0 & 0b111111) << 18);
                *self = Face(temp & 0b111_111_111_111_111_111_111_111);
            }
            Direction::CounterClockwise => {
                let temp = (self.0 << 6) | (self.0 >> 18);
                *self = Face(temp & 0b111_111_111_111_111_111_111_111);
            }
        }
    }

    // const fn reverse_edge(old: u32) -> u32 {
    //     (old >> 6 & 0b000_000_111) | (old & 0b000_111_000) | (old << 6 & 0b111_000_000)
    // }

    const fn get_edge(self, edge: Edge) -> u32 {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BasicCube {
    faces: [Face; 6],
}

impl BasicCube {
    pub fn solved() -> Self {
        Self {
            faces: [
                Face::from_array(&[Color::White; 8]),
                Face::from_array(&[Color::Orange; 8]),
                Face::from_array(&[Color::Green; 8]),
                Face::from_array(&[Color::Yellow; 8]),
                Face::from_array(&[Color::Red; 8]),
                Face::from_array(&[Color::Blue; 8]),
            ],
        }
    }

    pub fn rotate(&mut self, action: Rotate) {
        use Edge as E;

        let s1: (usize, Direction);
        let s2: [(usize, E); 4];

        match action {
            // Face Turns
            Rotate::U => {
                s1 = (0, Direction::Clockwise);
                s2 = [(1, E::Top), (5, E::Top), (4, E::Bottom), (2, E::Top)];
            }
            Rotate::Up => {
                s1 = (0, Direction::CounterClockwise);
                s2 = [(1, E::Top), (2, E::Top), (4, E::Bottom), (5, E::Top)];
            }
            Rotate::D => {
                s1 = (3, Direction::Clockwise);
                s2 = [(1, E::Bottom), (2, E::Bottom), (4, E::Top), (5, E::Bottom)];
            }
            Rotate::Dp => {
                s1 = (3, Direction::CounterClockwise);
                s2 = [(1, E::Bottom), (5, E::Bottom), (4, E::Top), (2, E::Bottom)];
            }
            Rotate::R => {
                s1 = (2, Direction::Clockwise);
                s2 = [(0, E::Right), (4, E::Right), (3, E::Right), (1, E::Right)];
            }
            Rotate::Rp => {
                s1 = (2, Direction::CounterClockwise);
                s2 = [(0, E::Right), (1, E::Right), (3, E::Right), (4, E::Right)];
            }
            Rotate::L => {
                s1 = (5, Direction::Clockwise);
                s2 = [(0, E::Left), (1, E::Left), (3, E::Left), (4, E::Left)];
            }
            Rotate::Lp => {
                s1 = (5, Direction::CounterClockwise);
                s2 = [(0, E::Left), (4, E::Left), (3, E::Left), (1, E::Left)];
            }
            Rotate::F => {
                s1 = (1, Direction::Clockwise);
                s2 = [(0, E::Bottom), (2, E::Left), (3, E::Top), (5, E::Right)];
            }
            Rotate::Fp => {
                s1 = (1, Direction::CounterClockwise);
                s2 = [(0, E::Bottom), (5, E::Right), (3, E::Top), (2, E::Left)];
            }
            Rotate::B => {
                s1 = (4, Direction::Clockwise);
                s2 = [(0, E::Top), (5, E::Left), (3, E::Bottom), (2, E::Right)];
            }
            Rotate::Bp => {
                s1 = (4, Direction::CounterClockwise);
                s2 = [(0, E::Top), (2, E::Right), (3, E::Bottom), (5, E::Left)];
            }
            _ => return,
        }

        // Step 1 rotate pieces on face
        self.faces[s1.0].rotate_mut(s1.1);

        // Step 2 move pieces between faces
        let a = self.faces[s2[0].0].get_edge(s2[0].1);
        let b = self.faces[s2[1].0].set_edge_mut(s2[1].1, a);
        let c = self.faces[s2[2].0].set_edge_mut(s2[2].1, b);
        let d = self.faces[s2[3].0].set_edge_mut(s2[3].1, c);
        self.faces[s2[0].0].set_edge_mut(s2[0].1, d);
    }

    pub fn rotated(&self, action: Rotate) -> Self {
        let mut cpy = self.clone();
        cpy.rotate(action);
        cpy
    }

    pub fn apply_rotations(&mut self, actions: &[Rotate]) {
        for &a in actions {
            self.rotate(a);
        }
    }

    pub fn print(&self) {
        let offsets = [(10, 5), (10, 9), (18, 9), (10, 13), (10, 1), (2, 9)];
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
        // can optimize by using bytes then transmuting to a str
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

    pub fn get_steps(&self, target: &Self) -> Vec<Rotate> {
        let possible_moves = [
            Rotate::U,
            Rotate::D,
            Rotate::R,
            Rotate::L,
            Rotate::F,
            Rotate::B,
            Rotate::Up,
            Rotate::Dp,
            Rotate::Rp,
            Rotate::Lp,
            Rotate::Fp,
            Rotate::Bp,
        ];

        // expansion from current state
        let mut seen_lhs = HashMap::new();
        seen_lhs.insert(self.clone(), Rotate::None);
        let mut states_lhs = vec![self.clone()];

        // expansion from target state
        let mut seen_rhs = HashMap::new();
        seen_rhs.insert(target.clone(), Rotate::None);
        let mut states_rhs = vec![target.clone()];

        let mut new_states = Vec::new();

        let mut states_explored: usize = 1;
        let mut depth: usize = 1;

        let (sl, sr, act) = 'main: loop {
            print!("exploring depth {}. lhs. ", depth);
            // expand search from the left
            new_states.clear();
            while let Some(start_state) = states_lhs.pop() {
                for r in possible_moves {
                    let new_state = start_state.rotated(r);

                    if seen_rhs.contains_key(&new_state) {
                        break 'main (start_state, new_state, r);
                    }

                    if seen_lhs.contains_key(&new_state) {
                        continue;
                    } else {
                        states_explored += 1;
                        seen_lhs.insert(new_state.clone(), r);
                        new_states.push(new_state);
                    }
                }
            }
            std::mem::swap(&mut states_lhs, &mut new_states);

            println!("rhs.");
            // expand search from the right
            new_states.clear();
            while let Some(start_state) = states_rhs.pop() {
                for r in possible_moves {
                    let new_state = start_state.rotated(r);

                    if seen_lhs.contains_key(&new_state) {
                        break 'main (new_state, start_state, r.prime());
                    }

                    if seen_rhs.contains_key(&new_state) {
                        continue;
                    } else {
                        states_explored += 1;
                        seen_rhs.insert(new_state.clone(), r);
                        new_states.push(new_state);
                    }
                }
            }
            std::mem::swap(&mut states_rhs, &mut new_states);

            depth += 1;
        };

        let mut moves_trace = vec![act];
        let mut current_trace_state = sl;
        while let Some(s) = seen_lhs.get(&current_trace_state) {
            if *s == Rotate::None {
                break;
            }
            moves_trace.push(*s);
            current_trace_state.rotate(s.prime());
        }
        moves_trace.reverse();
        current_trace_state = sr;
        while let Some(s) = seen_rhs.get(&current_trace_state) {
            if *s == Rotate::None {
                break;
            }
            moves_trace.push(s.prime());
            current_trace_state.rotate(s.prime());
        }

        println!("explored {} states", states_explored);
        moves_trace
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cube {
    internal_cube: BasicCube,
    mapping: [(usize, i8); 6],
}

impl Cube {
    pub fn solved() -> Self {
        Self {
            internal_cube: BasicCube::solved(),
            mapping: [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
        }
    }

    pub fn rotate(&mut self, action: Rotate) {
        use Edge as E;

        // println!("{action:?}");

        // let mapping = [0, 1, 2, 3, 4, 5];
        // let mapping = [1, 3, 2, 5, 4, 0];
        // let mapping = [5, 0, 2, 1, 4, 3];

        let s1: (usize, Direction);
        let s2: [(usize, E); 4];

        match action {
            Rotate::None => return,

            // Cube Rotations
            Rotate::x => {
                let temp = self.mapping[0];
                self.mapping[0] = self.mapping[1];
                self.mapping[1] = self.mapping[3];
                self.mapping[3] = self.mapping[4];
                self.mapping[4] = temp;
                self.mapping[5].1 += 1;
                self.mapping[2].1 -= 1;
                return;
            }
            Rotate::xp => {
                let temp = self.mapping[0];
                self.mapping[0] = self.mapping[4];
                self.mapping[4] = self.mapping[3];
                self.mapping[3] = self.mapping[1];
                self.mapping[1] = temp;
                self.mapping[2].1 += 1;
                self.mapping[5].1 -= 1;
                return;
            }
            Rotate::y => {
                let temp = self.mapping[1];
                self.mapping[1] = self.mapping[2];
                self.mapping[2] = self.mapping[4];
                self.mapping[4] = self.mapping[5];
                self.mapping[5] = temp;
                self.mapping[3].1 += 1;
                self.mapping[0].1 -= 1;

                self.mapping[2].1 += 2;
                self.mapping[4].1 += 2;
                return;
            }
            Rotate::yp => {
                let temp = self.mapping[1];
                self.mapping[1] = self.mapping[5];
                self.mapping[5] = self.mapping[4];
                self.mapping[4] = self.mapping[2];
                self.mapping[2] = temp;
                self.mapping[0].1 += 1;
                self.mapping[3].1 -= 1;

                self.mapping[4].1 += 2;
                self.mapping[5].1 += 2;
                return;
            }
            Rotate::z => {
                let temp = self.mapping[0];
                self.mapping[0] = self.mapping[5];
                self.mapping[5] = self.mapping[3];
                self.mapping[3] = self.mapping[2];
                self.mapping[2] = temp;
                self.mapping[1].1 -= 1;
                self.mapping[4].1 += 1;

                self.mapping[0].1 -= 1;
                self.mapping[2].1 -= 1;
                self.mapping[3].1 -= 1;
                self.mapping[5].1 -= 1;
                return;
            }
            Rotate::zp => {
                let temp = self.mapping[0];
                self.mapping[0] = self.mapping[2];
                self.mapping[2] = self.mapping[3];
                self.mapping[3] = self.mapping[5];
                self.mapping[5] = temp;
                self.mapping[1].1 += 1;
                self.mapping[4].1 -= 1;

                self.mapping[0].1 += 1;
                self.mapping[2].1 += 1;
                self.mapping[3].1 += 1;
                self.mapping[5].1 += 1;
                return;
            }

            // Slice Moves
            Rotate::M => {
                self.apply_rotations(&[Rotate::R, Rotate::Lp, Rotate::xp]);
                return;
            }
            Rotate::Mp => {
                self.apply_rotations(&[Rotate::Rp, Rotate::L, Rotate::x]);
                return;
            }
            Rotate::E => {
                self.apply_rotations(&[Rotate::U, Rotate::Dp, Rotate::yp]);
                return;
            }
            Rotate::Ep => {
                self.apply_rotations(&[Rotate::Up, Rotate::D, Rotate::y]);
                return;
            }
            Rotate::S => {
                self.apply_rotations(&[Rotate::B, Rotate::Fp, Rotate::z]);
                return;
            }
            Rotate::Sp => {
                self.apply_rotations(&[Rotate::Bp, Rotate::F, Rotate::zp]);
                return;
            }

            // Wide Moves
            Rotate::Uw => {
                self.apply_rotations(&[Rotate::D, Rotate::y]);
                return;
            }
            Rotate::Uwp => {
                self.apply_rotations(&[Rotate::Dp, Rotate::yp]);
                return;
            }
            Rotate::Dw => {
                self.apply_rotations(&[Rotate::U, Rotate::yp]);
                return;
            }
            Rotate::Dwp => {
                self.apply_rotations(&[Rotate::Up, Rotate::y]);
                return;
            }
            Rotate::Rw => {
                self.apply_rotations(&[Rotate::L, Rotate::x]);
                return;
            }
            Rotate::Rwp => {
                self.apply_rotations(&[Rotate::Lp, Rotate::xp]);
                return;
            }
            Rotate::Lw => {
                self.apply_rotations(&[Rotate::R, Rotate::xp]);
                return;
            }
            Rotate::Lwp => {
                self.apply_rotations(&[Rotate::Rp, Rotate::x]);
                return;
            }
            Rotate::Fw => {
                self.apply_rotations(&[Rotate::B, Rotate::z]);
                return;
            }
            Rotate::Fwp => {
                self.apply_rotations(&[Rotate::Bp, Rotate::zp]);
                return;
            }
            Rotate::Bw => {
                self.apply_rotations(&[Rotate::F, Rotate::zp]);
                return;
            }
            Rotate::Bwp => {
                self.apply_rotations(&[Rotate::Fp, Rotate::z]);
                return;
            }

            // Face Turns
            Rotate::U => {
                s1 = (0, Direction::Clockwise);
                s2 = [(1, E::Top), (5, E::Top), (4, E::Bottom), (2, E::Top)];
            }
            Rotate::Up => {
                s1 = (0, Direction::CounterClockwise);
                s2 = [(1, E::Top), (2, E::Top), (4, E::Bottom), (5, E::Top)];
            }
            Rotate::D => {
                s1 = (3, Direction::Clockwise);
                s2 = [(1, E::Bottom), (2, E::Bottom), (4, E::Top), (5, E::Bottom)];
            }
            Rotate::Dp => {
                s1 = (3, Direction::CounterClockwise);
                s2 = [(1, E::Bottom), (5, E::Bottom), (4, E::Top), (2, E::Bottom)];
            }
            Rotate::R => {
                s1 = (2, Direction::Clockwise);
                s2 = [(0, E::Right), (4, E::Right), (3, E::Right), (1, E::Right)];
            }
            Rotate::Rp => {
                s1 = (2, Direction::CounterClockwise);
                s2 = [(0, E::Right), (1, E::Right), (3, E::Right), (4, E::Right)];
            }
            Rotate::L => {
                s1 = (5, Direction::Clockwise);
                s2 = [(0, E::Left), (1, E::Left), (3, E::Left), (4, E::Left)];
            }
            Rotate::Lp => {
                s1 = (5, Direction::CounterClockwise);
                s2 = [(0, E::Left), (4, E::Left), (3, E::Left), (1, E::Left)];
            }
            Rotate::F => {
                s1 = (1, Direction::Clockwise);
                s2 = [(0, E::Bottom), (2, E::Left), (3, E::Top), (5, E::Right)];
            }
            Rotate::Fp => {
                s1 = (1, Direction::CounterClockwise);
                s2 = [(0, E::Bottom), (5, E::Right), (3, E::Top), (2, E::Left)];
            }
            Rotate::B => {
                s1 = (4, Direction::Clockwise);
                s2 = [(0, E::Top), (5, E::Left), (3, E::Bottom), (2, E::Right)];
            }
            Rotate::Bp => {
                s1 = (4, Direction::CounterClockwise);
                s2 = [(0, E::Top), (2, E::Right), (3, E::Bottom), (5, E::Left)];
            }
        }

        // Step 1 rotate pieces on face
        self.internal_cube.faces[self.mapping[s1.0].0].rotate_mut(s1.1);

        // Step 2 move pieces between internal_cube.faces
        let a = self.internal_cube.faces[self.mapping[s2[0].0].0]
            .get_edge(s2[0].1.rotate(self.mapping[s2[0].0].1));
        let b = self.internal_cube.faces[self.mapping[s2[1].0].0]
            .set_edge_mut(s2[1].1.rotate(self.mapping[s2[1].0].1), a);
        let c = self.internal_cube.faces[self.mapping[s2[2].0].0]
            .set_edge_mut(s2[2].1.rotate(self.mapping[s2[2].0].1), b);
        let d = self.internal_cube.faces[self.mapping[s2[3].0].0]
            .set_edge_mut(s2[3].1.rotate(self.mapping[s2[3].0].1), c);
        self.internal_cube.faces[self.mapping[s2[0].0].0]
            .set_edge_mut(s2[0].1.rotate(self.mapping[s2[0].0].1), d);
    }

    pub fn apply_rotations(&mut self, actions: &[Rotate]) {
        for &a in actions {
            self.rotate(a);
        }
    }

    pub fn to_string(&self) -> String {
        let offsets = [(10, 5), (10, 9), (18, 9), (10, 13), (10, 1), (2, 9)];
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
        // can optimize by using bytes then transmuting to a str
        let colors = ['0', 'W', 'O', 'G', 'Y', 'R', 'B', '7'];
        let mut ac: [[char; 25]; 17] = ASCIICUBE.clone();

        // populate the proper colors
        // for (i, f) in self.faces.iter().enumerate() {
        for i in 0..6 {
            let i_m = self.mapping[i];
            let f = self.internal_cube.faces[i_m.0].rotated(-i_m.1);
            for sq in 0..8 {
                let x = offsets[i].0 + sq_off[sq].0;
                let y = offsets[i].1 + sq_off[sq].1;
                let c = ((f.0 >> ((7 - sq) * 3)) & 0b0111) as usize;
                ac[y][x] = colors[c];
            }
            ac[offsets[i].1 + 1][offsets[i].0 + 2] = colors[i_m.0 + 1];
        }

        let mut s = String::with_capacity(25 * 18 + 1 + 3);
        for line in ac {
            for ch in line {
                s.push(ch);
            }
            s.push('\n');
        }
        s
    }

    pub fn print_normalized(&self) {
        self.internal_cube.print();
    }

    pub fn get_steps(&self, target: &Cube) -> Vec<Rotate> {
        // get steps to orient cube
        // TODO:
        let mut moves = Vec::new();

        // get steps to solve cube
        moves.append(&mut self.internal_cube.get_steps(&target.internal_cube));

        moves
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
        cube.print_normalized();

        let mut cube2 = Cube::solved();
        cube2.rotate(Rotate::R);
        cube2.rotate(Rotate::F);
        cube2.print_normalized();

        assert_eq!(cube.internal_cube, cube2.internal_cube);
    }

    #[test]
    fn t2() {
        let mut cube = Cube::solved();
        cube.rotate(Rotate::Mp);
        cube.rotate(Rotate::U);
        cube.rotate(Rotate::Mp);
        cube.print_normalized();

        let mut cube2 = Cube::solved();
        cube2.rotate(Rotate::Rp);
        cube2.rotate(Rotate::L);
        cube2.rotate(Rotate::F);
        cube2.rotate(Rotate::Rp);
        cube2.rotate(Rotate::L);
        cube2.print_normalized();

        assert_eq!(cube.internal_cube, cube2.internal_cube);
    }

    #[test]
    fn basic_scramble_5mv_1000x() {
        let mv = 5;
        let reps = 1000;
        let solved = Cube::solved();

        for _ in 0..reps {
            let mut cube = Cube::solved();
            for _ in 0..mv {
                cube.rotate(Rotate::from_num(rand::random_range(0..12)));
            }
            cube.apply_rotations(&cube.get_steps(&solved));
            assert_eq!(cube.internal_cube, solved.internal_cube);
        }
    }

    #[test]
    fn adv_scramble_5mv_1000x() {
        let mv = 5;
        let reps = 1000;
        let solved = Cube::solved();

        for _ in 0..reps {
            let mut cube = Cube::solved();
            for _ in 0..mv {
                cube.rotate(Rotate::from_num(rand::random_range(0..35)));
            }
            // must apply to internal cube to avoid dealing with rotations
            cube.internal_cube.apply_rotations(&cube.get_steps(&solved));

            assert_eq!(cube.internal_cube, solved.internal_cube);
        }
    }

    #[test]
    fn basic_scramble_10mv_20x() {
        let mv = 10;
        let reps = 20;
        let solved = Cube::solved();

        for _ in 0..reps {
            let mut cube = Cube::solved();
            for _ in 0..mv {
                cube.rotate(Rotate::from_num(rand::random_range(0..12)));
            }
            cube.apply_rotations(&cube.get_steps(&solved));
            assert_eq!(cube.internal_cube, solved.internal_cube);
        }
    }

    #[test]
    fn all_moves() {
        let mut cube = Cube::solved();
        cube.apply_rotations(&[
            Rotate::R,
            Rotate::U,
            Rotate::Lp,
            Rotate::D,
            Rotate::F,
            Rotate::U,
            Rotate::Rp,
            Rotate::B,
            Rotate::Up,
            Rotate::F,
        ]);
        cube.print_normalized();

        use crate::cube::Color as C;

        let target = Cube {
            internal_cube: BasicCube {
                faces: [
                    Face::from_array(&[
                        C::Orange,
                        C::White,
                        C::Yellow,
                        C::White,
                        C::Red,
                        C::Red,
                        C::Red,
                        C::Orange,
                    ]),
                    Face::from_array(&[
                        C::Blue,
                        C::Blue,
                        C::Green,
                        C::Yellow,
                        C::Yellow,
                        C::White,
                        C::White,
                        C::Yellow,
                    ]),
                    Face::from_array(&[
                        C::Yellow,
                        C::Red,
                        C::Green,
                        C::Red,
                        C::Red,
                        C::Orange,
                        C::Blue,
                        C::Blue,
                    ]),
                    Face::from_array(&[
                        C::Blue,
                        C::Blue,
                        C::Orange,
                        C::Green,
                        C::White,
                        C::Orange,
                        C::Orange,
                        C::Yellow,
                    ]),
                    Face::from_array(&[
                        C::Blue,
                        C::White,
                        C::Green,
                        C::Green,
                        C::Orange,
                        C::Green,
                        C::Green,
                        C::Blue,
                    ]),
                    Face::from_array(&[
                        C::White,
                        C::Yellow,
                        C::Yellow,
                        C::Green,
                        C::Red,
                        C::Red,
                        C::White,
                        C::Orange,
                    ]),
                ],
            },
            mapping: [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
        };
        target.print_normalized();

        assert_eq!(cube.internal_cube, target.internal_cube);
    }

    #[test]
    fn rubiks() {
        let mut cube = Cube::solved();
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
            // second half
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
        cube.print_normalized();

        let mut cube2 = Cube::solved();
        cube2.apply_rotations(&[
            Rotate::Mp,
            Rotate::Up,
            Rotate::Mp,
            Rotate::Up,
            Rotate::Mp,
            Rotate::Up,
            Rotate::Up,
            // second half
            Rotate::M,
            Rotate::Up,
            Rotate::M,
            Rotate::Up,
            Rotate::M,
            Rotate::Up,
            Rotate::Up,
        ]);
        cube2.print_normalized();

        assert_eq!(cube.internal_cube, cube2.internal_cube);
    }
}
