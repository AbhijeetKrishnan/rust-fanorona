use crate::{Piece, Direction};
use std::fmt;

use crate::Square;


const BB_EMPTY: u64 = 0x0; // empty board
const BB_ALL: u64 = !0x0; // full board

// per-square masks
const BB_A1: u64 = 0x100000000000;
const BB_A2: u64 = 0x800000000;
const BB_A3: u64 = 0x4000000;
const BB_A4: u64 = 0x20000;
const BB_A5: u64 = 0x100;
const BB_B1: u64 = 0x80000000000;
const BB_B2: u64 = 0x400000000;
const BB_B3: u64 = 0x2000000;
const BB_B4: u64 = 0x10000;
const BB_B5: u64 = 0x80;
const BB_C1: u64 = 0x40000000000;
const BB_C2: u64 = 0x200000000;
const BB_C3: u64 = 0x1000000;
const BB_C4: u64 = 0x8000;
const BB_C5: u64 = 0x40;
const BB_D1: u64 = 0x20000000000;
const BB_D2: u64 = 0x100000000;
const BB_D3: u64 = 0x800000;
const BB_D4: u64 = 0x4000;
const BB_D5: u64 = 0x20;
const BB_E1: u64 = 0x10000000000;
const BB_E2: u64 = 0x80000000;
const BB_E3: u64 = 0x400000;
const BB_E4: u64 = 0x2000;
const BB_E5: u64 = 0x10;
const BB_F1: u64 = 0x8000000000;
const BB_F2: u64 = 0x40000000;
const BB_F3: u64 = 0x200000;
const BB_F4: u64 = 0x1000;
const BB_F5: u64 = 0x8;
const BB_G1: u64 = 0x4000000000;
const BB_G2: u64 = 0x20000000;
const BB_G3: u64 = 0x100000;
const BB_G4: u64 = 0x800;
const BB_G5: u64 = 0x4;
const BB_H1: u64 = 0x2000000000;
const BB_H2: u64 = 0x10000000;
const BB_H3: u64 = 0x80000;
const BB_H4: u64 = 0x400;
const BB_H5: u64 = 0x2;
const BB_I1: u64 = 0x1000000000;
const BB_I2: u64 = 0x8000000;
const BB_I3: u64 = 0x40000;
const BB_I4: u64 = 0x200;
const BB_I5: u64 = 0x1;

// BB_POS[sq] gives mask for square sq (A1 is 0, I5 is 45)
const BB_POS: [u64; 45] = [ 
    BB_A1, BB_B1, BB_C1, BB_D1, BB_E1, BB_F1, BB_G1, BB_H1, BB_I1,
    BB_A2, BB_B2, BB_C2, BB_D2, BB_E2, BB_F2, BB_G2, BB_H2, BB_I2,
    BB_A3, BB_B3, BB_C3, BB_D3, BB_E3, BB_F3, BB_G3, BB_H3, BB_I3,
    BB_A4, BB_B4, BB_C4, BB_D4, BB_E4, BB_F4, BB_G4, BB_H4, BB_I4,
    BB_A5, BB_B5, BB_C5, BB_D5, BB_E5, BB_F5, BB_G5, BB_H5, BB_I5,
];

// BB_MOVES[sq] gives mask for legal squares that can be moved to from sq
// TODO: find way to generate this via const function
const BB_MOVES: [u64; 45] = [
    BB_A2 | BB_B1 | BB_B2,
    BB_A1 | BB_B2 | BB_C1,
    BB_B1 | BB_B2 | BB_C2 | BB_D1 | BB_D2,
    BB_C1 | BB_D2 | BB_E1,
    BB_D1 | BB_D2 | BB_E2 | BB_F1 | BB_F2,
    BB_E1 | BB_F2 | BB_G1,
    BB_F1 | BB_F2 | BB_G2 | BB_H1 | BB_H2,
    BB_G1 | BB_H2 | BB_I1,
    BB_H1 | BB_H2 | BB_I2,

    BB_A1 | BB_A3 | BB_B2,
    BB_A1 | BB_A2 | BB_A3 | BB_B1 | BB_B3 | BB_C1 | BB_C2 | BB_C3,
    BB_B2 | BB_C1 | BB_C3 | BB_D2,
    BB_C1 | BB_C2 | BB_C3 | BB_D1 | BB_D3 | BB_E1 | BB_E2 | BB_E3,
    BB_D2 | BB_E1 | BB_E3 | BB_F2,
    BB_E1 | BB_E2 | BB_E3 | BB_F1 | BB_F3 | BB_G1 | BB_G2 | BB_G3,
    BB_F2 | BB_G1 | BB_G3 | BB_H2,
    BB_G1 | BB_G2 | BB_G3 | BB_H1 | BB_H3 | BB_I1 | BB_I2 | BB_I3,
    BB_H2 | BB_I1 | BB_I3,

    BB_A2 | BB_A4 | BB_B2 | BB_B3 | BB_B4,
    BB_A3 | BB_B2 | BB_B4 | BB_C3,
    BB_B2 | BB_B3 | BB_B4 | BB_C2 | BB_C4 | BB_D2 | BB_D3 | BB_D4,
    BB_C3 | BB_D2 | BB_D4 | BB_E3,
    BB_D2 | BB_D3 | BB_D4 | BB_E2 | BB_E4 | BB_F2 | BB_F3 | BB_F4,
    BB_E3 | BB_F2 | BB_F4 | BB_G3,
    BB_F2 | BB_F3 | BB_F4 | BB_G2 | BB_G4 | BB_H2 | BB_H3 | BB_H4,
    BB_G3 | BB_H2 | BB_H4 | BB_I3,
    BB_H2 | BB_H3 | BB_H4 | BB_I2 | BB_I4,

    BB_A3 | BB_A5 | BB_B4,
    BB_A3 | BB_A4 | BB_A5 | BB_B3 | BB_B5 | BB_C3 | BB_C4 | BB_C5,
    BB_B4 | BB_C3 | BB_C5 | BB_D4,
    BB_C3 | BB_C4 | BB_C5 | BB_D3 | BB_D5 | BB_E3 | BB_E4 | BB_E5,
    BB_D4 | BB_E3 | BB_E5 | BB_F4,
    BB_E3 | BB_E4 | BB_E5 | BB_F3 | BB_F5 | BB_G3 | BB_G4 | BB_G5,
    BB_F4 | BB_G3 | BB_G5 | BB_H4,
    BB_G3 | BB_G4 | BB_G5 | BB_H3 | BB_H5 | BB_I3 | BB_I4 | BB_I5,
    BB_H4 | BB_I3 | BB_I5,

    BB_A4 | BB_B4 | BB_B5,
    BB_A5 | BB_B4 | BB_C5,
    BB_B4 | BB_B5 | BB_C5 | BB_D4 | BB_D5,
    BB_C5 | BB_D4 | BB_E5,
    BB_D4 | BB_D5 | BB_E4 | BB_F4 | BB_F5,
    BB_E5 | BB_F4 | BB_G5,
    BB_F4 | BB_F5 | BB_G4 | BB_H4 | BB_H5,
    BB_G5 | BB_H4 | BB_I5,
    BB_H4 | BB_H5 | BB_I4,
];

const BB_ROW: [u64; 5] = [0x1ff000000000, 0xff8000000, 0x7fc0000, 0x3fe00, 0x1ff];
const BB_COL: [u64; 9] = [
    0x100804020100,
    0x80402010080,
    0x40201008040,
    0x20100804020,
    0x10080402010,
    0x8040201008,
    0x4020100804,
    0x2010080402,
    0x1008040201,
];

const BB_BLACK: u64 = 0x1ffffd280000;
const BB_WHITE: u64 = 0x297ffff;

#[inline]
const fn ray(square: Square, direction: Direction) -> u64 {
    todo!()
}

const BIT_PATTERN_TO_LOG2: [i8; 128] = [
    0, // change to 1 if you want bitSize(0) = 1
    48, -1, -1, 31, -1, 15, 51, -1, 63, 5, -1, -1, -1, 19, -1, 
    23, 28, -1, -1, -1, 40, 36, 46, -1, 13, -1, -1, -1, 34, -1, 58,
    -1, 60, 2, 43, 55, -1, -1, -1, 50, 62, 4, -1, 18, 27, -1, 39, 
    45, -1, -1, 33, 57, -1, 1, 54, -1, 49, -1, 17, -1, -1, 32, -1,
    53, -1, 16, -1, -1, 52, -1, -1, -1, 64, 6, 7, 8, -1, 9, -1, 
    -1, -1, 20, 10, -1, -1, 24, -1, 29, -1, -1, 21, -1, 11, -1, -1,
    41, -1, 25, 37, -1, 47, -1, 30, 14, -1, -1, -1, -1, 22, -1, -1,
    35, 12, -1, -1, -1, 59, 42, -1, -1, 61, 3, 26, 38, 44, -1, 56
];
const MULTIPLICATOR: u64 = 0x6c04f118e9966f6b;

const fn msb(mut v: u64) -> u8 {
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v |= v >> 32;
    BIT_PATTERN_TO_LOG2[((v * MULTIPLICATOR) as u64 >> 57) as usize] as u8
}

#[test]
fn test_msb() {
    assert_eq!(0, msb(0x0));
    assert_eq!(1, msb(0x1));
    for square in 0..45 {
        assert_eq!(square as u8, msb(BB_POS[square]));
    }
}

pub struct BaseBoard {
    black: u64,
    white: u64,
}

impl BaseBoard {
    pub fn new() -> BaseBoard {
        BaseBoard {
            black: BB_BLACK,
            white: BB_WHITE,
        }
    }

    fn remove_piece_at(&mut self, at: Square) {
        self.black &= !BB_POS[at.idx()];
        self.white &= !BB_POS[at.idx()];
    }

    fn set_piece_at(&mut self, piece: Piece, at: Square) {
        match piece {
            Piece::BLACK => self.black |= BB_POS[at.idx()],
            Piece::WHITE => self.white |= BB_POS[at.idx()],
        }
    }
}

impl fmt::Display for BaseBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_chars = [['.'; 9]; 5];
        for row in 0..5 {
            for col in 0..9 {
                if self.white & BB_POS[row * 9 + col] > 0 {
                    board_chars[row][col] = 'W';
                } else if self.black & BB_POS[row * 9 + col] > 0 {
                    board_chars[row][col] = 'B';
                }
            }
        }
        write!(
            f,
            "{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
            board_chars[0], board_chars[1], board_chars[2], board_chars[3], board_chars[4]
        )
    }
}
