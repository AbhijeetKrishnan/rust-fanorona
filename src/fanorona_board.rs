use std::fmt;

use crate::FanoronaMove;

const BB_EMPTY: u64 = 0;
const BB_ALL: u64 = !0;
const BB_POS: [[u64; 9]; 5] = [
    [0x100000000000, 0x80000000000, 0x40000000000, 0x20000000000, 0x10000000000, 0x8000000000, 0x4000000000, 0x2000000000, 0x1000000000],
    [   0x800000000,   0x400000000,   0x200000000,   0x100000000,    0x80000000,   0x40000000,   0x20000000,   0x10000000,    0x8000000],
    [     0x4000000,     0x2000000,     0x1000000,      0x800000,      0x400000,     0x200000,     0x100000,      0x80000,      0x40000],
    [       0x20000,       0x10000,        0x8000,        0x4000,        0x2000,       0x1000,        0x800,        0x400,        0x200],
    [         0x100,          0x80,          0x40,          0x20,          0x10,          0x8,          0x4,          0x2,          0x1],
];
const BB_ROW: [u64; 5] = [0x1ff000000000, 0xff8000000, 0x7fc0000, 0x3fe00, 0x1ff];
const BB_COL: [u64; 9] = [0x100804020100, 0x80402010080, 0x40201008040, 0x20100804020, 0x10080402010, 0x8040201008, 0x4020100804, 0x2010080402, 0x1008040201];

pub struct BaseBoard {
    black: u64,
    white: u64,
}

impl BaseBoard {
    pub fn new() -> BaseBoard {
        BaseBoard {
            black: 0x1ffffd280000,
            white: 0x297ffff,
        }
    }

    pub fn push(&mut self, fmove: FanoronaMove) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    pub fn push_str(&mut self, fmove_str: &'static str) -> Result<(), Box<dyn std::error::Error>> {
        let fmove = FanoronaMove::parse_move_str(fmove_str).ok_or_else(|| Box::<dyn std::error::Error>::from("could not parse move"))?;
        self.push(fmove)
    }
}

impl fmt::Display for BaseBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_chars = [['.'; 9]; 5];
        for row in 0..5 {
            for col in 0..9 {
                if self.white & BB_POS[row][col] > 0 {
                    board_chars[row][col] = 'W';
                } else if self.black & BB_POS[row][col] > 0 {
                    board_chars[row][col] = 'B';
                }
            }
        }
        write!(f, "{:?}\n{:?}\n{:?}\n{:?}\n{:?}", board_chars[0], board_chars[1], board_chars[2], board_chars[3], board_chars[4])
    }
}