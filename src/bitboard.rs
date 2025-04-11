use crate::square::SquareIterator;
use crate::Direction;
use crate::FanoronaError;
use std::cmp::Ordering;
use std::fmt;
use std::ops;

use crate::Square;

pub const UNUSED: usize = 19;
pub const ROWS: usize = 5;
pub const COLS: usize = 9;

/// A bitboard representation of a Fanorona board position as an unsigned 64-bit integer
///
/// Lower order bits 0 to 44 represent the positions occupied by pieces of a single color. If bit i in the bitboard for
/// white pieces is set, then square i contains a white piece, otherwise it is empty.
///
/// The square indices correspond to the layout of the board as follows -
///
/// ```text
///   [black] 36 37 38 39 40 41 42 43 44
///           27 28 29 30 31 32 33 34 35
///           18 19 20 21 22 23 24 25 26
///            9 10 11 12 13 14 15 16 17
///   [white]  0  1  2  3  4  5  6  7  8
/// ```
///
/// The 19 higher-order bits are currently not used for anything.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BitBoard(u64);

// implement all bitwise operators for operating on BitBoards
impl_op_ex!(!|bb: &BitBoard| -> BitBoard { BitBoard(!bb.0) });
impl_op_ex!(&|bb1: &BitBoard, bb2: &BitBoard| -> BitBoard { BitBoard(bb1.0 & bb2.0) });
impl_op_ex!(| |bb1: &BitBoard, bb2: &BitBoard| -> BitBoard { BitBoard(bb1.0 | bb2.0) });
impl_op_ex!(^ |bb1: &BitBoard, bb2: &BitBoard| -> BitBoard { BitBoard(bb1.0 ^ bb2.0) });
impl_op_ex!(>> |bb: &BitBoard, shift: &usize| -> BitBoard { BitBoard(bb.0 >> shift) });
impl_op_ex!(<< |bb: &BitBoard, shift: &usize| -> BitBoard { BitBoard(bb.0 << shift) });
impl_op_ex!(&= |bb1: &mut BitBoard, bb2: &BitBoard| { bb1.0 &= bb2.0 });
impl_op_ex!(|= |bb1: &mut BitBoard, bb2: &BitBoard| { bb1.0 |= bb2.0 });
impl_op_ex_commutative!(&|bb1: &BitBoard, bb2: &u64| -> BitBoard { BitBoard(bb1.0 & bb2) });
impl_op_ex_commutative!(| |bb1: &BitBoard, bb2: &u64| -> BitBoard { BitBoard(bb1.0 | bb2) });

impl fmt::Display for BitBoard {
    /// Print a BitBoard as a 5x9 grid of the bits
    /// # Example
    /// ```text
    /// 0 0 0 0 0 0 0 0 0
    /// 0 0 0 0 0 0 0 0 0
    /// 0 1 0 1 0 0 1 0 1
    /// 1 1 1 1 1 1 1 1 1
    /// 1 1 1 1 1 1 1 1 1
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0COLS$b}\n{:0COLS$b}\n{:0COLS$b}\n{:0COLS$b}\n{:0COLS$b}",
            (self.0.reverse_bits() >> UNUSED) & 0b111111111,
            (self.0.reverse_bits() >> (UNUSED + COLS)) & 0b111111111,
            (self.0.reverse_bits() >> (UNUSED + COLS * 2)) & 0b111111111,
            (self.0.reverse_bits() >> (UNUSED + COLS * 3)) & 0b111111111,
            (self.0.reverse_bits() >> (UNUSED + COLS * 4)) & 0b111111111,
        )
    }
}

impl PartialEq<u64> for BitBoard {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u64> for BitBoard {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        if self.0 < *other {
            Some(Ordering::Less)
        } else if self.0 == *other {
            Some(Ordering::Equal)
        } else if self.0 > *other {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl BitBoard {
    /// Get the bitmask corresponding to the ray of squares given an initial square and direction
    ///
    /// # Example
    /// `ray(Square(12), Direction::NorthEast)` =
    /// ```text
    ///   0 0 0 0 0 0 1 0 0
    ///   0 0 0 0 0 1 0 0 0
    ///   0 0 0 0 1 0 0 0 0
    ///   0 0 0 1 0 0 0 0 0
    ///   0 0 0 0 0 0 0 0 0
    /// ```
    #[inline]
    pub fn ray(square: Square, direction: Direction) -> BitBoard {
        BB_RAY[square][direction]
    }

    /// Get the bitmask corresponding to a particular square
    #[inline]
    pub fn pos(square: Square) -> BitBoard {
        BitBoard(1 << <Square as Into<usize>>::into(square))
    }

    /// Get the bitmask corresponding to the ray of squares containing a contiguous sequence of opponent pieces in the
    /// direction of the ray
    ///
    /// Allows for the implementation of capture moves
    pub fn get_capture_mask(
        opponent_bb: BitBoard,
        ray_start: Square,
        ray_dir: Direction,
    ) -> BitBoard {
        let ray_mask = opponent_bb & BitBoard::ray(ray_start, ray_dir);
        let mut bb = BitBoard(0x0);
        for square in SquareIterator::new(ray_start, ray_dir) {
            let bit = BB_POS[square] & ray_mask;
            if bit == 0 {
                break;
            } else {
                bb |= bit;
            }
        }
        bb
    }

    /// Get the list of `Squares` that are set in the current `BitBoard`
    pub fn as_squares(&self) -> Vec<Square> {
        let mut squares: Vec<Square> = vec![];
        for i in 0..ROWS * COLS {
            if self.0 >> i & 1 == 1 {
                squares.push(Square::from(i));
            }
        }
        squares
    }
}

impl TryFrom<&str> for BitBoard {
    type Error = FanoronaError;

    /// Parse a comma-separated list of squares as a bitboard. If the string is "-", return an empty bitboard
    ///
    /// Used to parse the list of squares visited during a capture sequence
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut bb = BitBoard(0x0);
        for sq_str in value.split(',') {
            let square = Square::try_from(sq_str)?;
            bb |= BB_POS[square];
        }
        Ok(bb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        println!("Black:\n{}", BB_BLACK);
        println!("White:\n{}", BB_WHITE);
    }

    #[test]
    fn test_get_capture_mask() {
        assert_eq!(
            BitBoard::get_capture_mask(BB_EMPTY, Square::new(0).unwrap(), Direction::NorthEast),
            BitBoard(0x0)
        );
    }

    #[test]
    fn test_ray() {
        assert_eq!(
            BitBoard::ray(Square::new(12).unwrap(), Direction::NorthEast),
            BitBoard(0x40100400000),
        );
        assert_eq!(
            BitBoard::ray(Square::new(44).unwrap(), Direction::NorthEast),
            BitBoard(0x0)
        );
    }

    #[test]
    fn test_as_squares() {
        assert_eq!(BB_EMPTY.as_squares(), vec![]);
        assert_eq!(BB_A1.as_squares(), vec![Square::from(0)]);
        assert_eq!(BB_I5.as_squares(), vec![Square::from(44)]);
    }

    #[test]
    fn test_try_from() {
        assert_eq!(
            BitBoard::try_from("A1,B2,C3").unwrap(),
            BB_A1 | BB_B2 | BB_C3
        );
        assert!(BitBoard::try_from("X1").is_err());
    }
}

/// Bitmask corresponding to an empty board
pub const BB_EMPTY: BitBoard = BitBoard(0x0); // empty board

// Bitmasks corresponding to an individual square
// TODO: possible to simplify this with a macro?
const BB_A1: BitBoard = BitBoard(1 << 0);
const BB_B1: BitBoard = BitBoard(1 << 1);
const BB_C1: BitBoard = BitBoard(1 << 2);
const BB_D1: BitBoard = BitBoard(1 << 3);
const BB_E1: BitBoard = BitBoard(1 << 4);
const BB_F1: BitBoard = BitBoard(1 << 5);
const BB_G1: BitBoard = BitBoard(1 << 6);
const BB_H1: BitBoard = BitBoard(1 << 7);
const BB_I1: BitBoard = BitBoard(1 << 8);
const BB_A2: BitBoard = BitBoard(1 << 9);
const BB_B2: BitBoard = BitBoard(1 << 10);
const BB_C2: BitBoard = BitBoard(1 << 11);
const BB_D2: BitBoard = BitBoard(1 << 12);
const BB_E2: BitBoard = BitBoard(1 << 13);
const BB_F2: BitBoard = BitBoard(1 << 14);
const BB_G2: BitBoard = BitBoard(1 << 15);
const BB_H2: BitBoard = BitBoard(1 << 16);
const BB_I2: BitBoard = BitBoard(1 << 17);
const BB_A3: BitBoard = BitBoard(1 << 18);
const BB_B3: BitBoard = BitBoard(1 << 19);
const BB_C3: BitBoard = BitBoard(1 << 20);
const BB_D3: BitBoard = BitBoard(1 << 21);
const BB_E3: BitBoard = BitBoard(1 << 22);
const BB_F3: BitBoard = BitBoard(1 << 23);
const BB_G3: BitBoard = BitBoard(1 << 24);
const BB_H3: BitBoard = BitBoard(1 << 25);
const BB_I3: BitBoard = BitBoard(1 << 26);
const BB_A4: BitBoard = BitBoard(1 << 27);
const BB_B4: BitBoard = BitBoard(1 << 28);
const BB_C4: BitBoard = BitBoard(1 << 29);
const BB_D4: BitBoard = BitBoard(1 << 30);
const BB_E4: BitBoard = BitBoard(1 << 31);
const BB_F4: BitBoard = BitBoard(1 << 32);
const BB_G4: BitBoard = BitBoard(1 << 33);
const BB_H4: BitBoard = BitBoard(1 << 34);
const BB_I4: BitBoard = BitBoard(1 << 35);
const BB_A5: BitBoard = BitBoard(1 << 36);
const BB_B5: BitBoard = BitBoard(1 << 37);
const BB_C5: BitBoard = BitBoard(1 << 38);
const BB_D5: BitBoard = BitBoard(1 << 39);
const BB_E5: BitBoard = BitBoard(1 << 40);
const BB_F5: BitBoard = BitBoard(1 << 41);
const BB_G5: BitBoard = BitBoard(1 << 42);
const BB_H5: BitBoard = BitBoard(1 << 43);
const BB_I5: BitBoard = BitBoard(1 << 44);

// BB_POS[sq] gives mask for square sq (A1 is 0, I5 is 44)
#[rustfmt::skip]
pub const BB_POS: [BitBoard; ROWS * COLS] = [
    BB_A1, BB_B1, BB_C1, BB_D1, BB_E1, BB_F1, BB_G1, BB_H1, BB_I1, 
    BB_A2, BB_B2, BB_C2, BB_D2, BB_E2, BB_F2, BB_G2, BB_H2, BB_I2, 
    BB_A3, BB_B3, BB_C3, BB_D3, BB_E3, BB_F3, BB_G3, BB_H3, BB_I3, 
    BB_A4, BB_B4, BB_C4, BB_D4, BB_E4, BB_F4, BB_G4, BB_H4, BB_I4, 
    BB_A5, BB_B5, BB_C5, BB_D5, BB_E5, BB_F5, BB_G5, BB_H5, BB_I5,
];

// BB_MOVES[sq] gives mask for legal squares that can be moved to from sq
// TODO: find way to generate this via const function
pub const BB_MOVES: [BitBoard; ROWS * COLS] = [
    BitBoard(0x602),
    BitBoard(0x405),
    BitBoard(0x1c0a),
    BitBoard(0x1014),
    BitBoard(0x7028),
    BitBoard(0x4050),
    BitBoard(0x1c0a0),
    BitBoard(0x10140),
    BitBoard(0x30080),
    BitBoard(0x40401),
    BitBoard(0x1c0a07),
    BitBoard(0x101404),
    BitBoard(0x70281c),
    BitBoard(0x405010),
    BitBoard(0x1c0a070),
    BitBoard(0x1014040),
    BitBoard(0x70281c0),
    BitBoard(0x4010100),
    BitBoard(0x18080600),
    BitBoard(0x10140400),
    BitBoard(0x70281c00),
    BitBoard(0x40501000),
    BitBoard(0x1c0a07000),
    BitBoard(0x101404000),
    BitBoard(0x70281c000),
    BitBoard(0x405010000),
    BitBoard(0xc02030000),
    BitBoard(0x1010040000),
    BitBoard(0x70281c0000),
    BitBoard(0x4050100000),
    BitBoard(0x1c0a0700000),
    BitBoard(0x10140400000),
    BitBoard(0x70281c00000),
    BitBoard(0x40501000000),
    BitBoard(0x1c0a07000000),
    BitBoard(0x100404000000),
    BitBoard(0x2018000000),
    BitBoard(0x5010000000),
    BitBoard(0xe050000000),
    BitBoard(0x14040000000),
    BitBoard(0x281c0000000),
    BitBoard(0x50100000000),
    BitBoard(0xa0700000000),
    BitBoard(0x140400000000),
    BitBoard(0x80c00000000),
];

/// Bitmasks for all squares along a row
pub const BB_ROW: [BitBoard; ROWS] = [
    BitBoard(0x1ff),          // row 1
    BitBoard(0x3fe00),        // row 2
    BitBoard(0x7fc0000),      // row 3
    BitBoard(0xff8000000),    // row 4
    BitBoard(0x1ff000000000), // row 5
];

/// Bitmasks for all squares along a column
pub const BB_COL: [BitBoard; COLS] = [
    BitBoard(0x1008040201),   // column A
    BitBoard(0x2010080402),   // column B
    BitBoard(0x4020100804),   // column C
    BitBoard(0x8040201008),   // column D
    BitBoard(0x10080402010),  // column E
    BitBoard(0x20100804020),  // column F
    BitBoard(0x40201008040),  // column G
    BitBoard(0x80402010080),  // column H
    BitBoard(0x100804020100), // column I
];

/// Bitmasks for all possible rays
///
/// `BB_RAY[square][direction]` gives the bitmask of all squares along `direction`, starting from `square`
///
/// generated using the `scripts/board_to_num.py` script
pub const BB_RAY: [[BitBoard; 8]; ROWS * COLS] = [
    [
        BitBoard(0x1008040200),
        BitBoard(0x10040100400),
        BitBoard(0x1fe),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x2010080400),
        BitBoard(0x0),
        BitBoard(0x1fc),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x1),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x4020100800),
        BitBoard(0x40100401000),
        BitBoard(0x1f8),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x3),
        BitBoard(0x40400),
    ],
    [
        BitBoard(0x8040201000),
        BitBoard(0x0),
        BitBoard(0x1f0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x7),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x10080402000),
        BitBoard(0x100401004000),
        BitBoard(0x1e0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0xf),
        BitBoard(0x1010101000),
    ],
    [
        BitBoard(0x20100804000),
        BitBoard(0x0),
        BitBoard(0x1c0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x1f),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x40201008000),
        BitBoard(0x4010000),
        BitBoard(0x180),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x3f),
        BitBoard(0x4040404000),
    ],
    [
        BitBoard(0x80402010000),
        BitBoard(0x0),
        BitBoard(0x100),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x7f),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x100804020000),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0xff),
        BitBoard(0x10101010000),
    ],
    [
        BitBoard(0x1008040000),
        BitBoard(0x0),
        BitBoard(0x3fc00),
        BitBoard(0x0),
        BitBoard(0x1),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x2010080000),
        BitBoard(0x10040100000),
        BitBoard(0x3f800),
        BitBoard(0x4),
        BitBoard(0x2),
        BitBoard(0x1),
        BitBoard(0x200),
        BitBoard(0x40000),
    ],
    [
        BitBoard(0x4020100000),
        BitBoard(0x0),
        BitBoard(0x3f000),
        BitBoard(0x0),
        BitBoard(0x4),
        BitBoard(0x0),
        BitBoard(0x600),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x8040200000),
        BitBoard(0x40100400000),
        BitBoard(0x3e000),
        BitBoard(0x10),
        BitBoard(0x8),
        BitBoard(0x4),
        BitBoard(0xe00),
        BitBoard(0x1010100000),
    ],
    [
        BitBoard(0x10080400000),
        BitBoard(0x0),
        BitBoard(0x3c000),
        BitBoard(0x0),
        BitBoard(0x10),
        BitBoard(0x0),
        BitBoard(0x1e00),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x20100800000),
        BitBoard(0x100401000000),
        BitBoard(0x38000),
        BitBoard(0x40),
        BitBoard(0x20),
        BitBoard(0x10),
        BitBoard(0x3e00),
        BitBoard(0x4040400000),
    ],
    [
        BitBoard(0x40201000000),
        BitBoard(0x0),
        BitBoard(0x30000),
        BitBoard(0x0),
        BitBoard(0x40),
        BitBoard(0x0),
        BitBoard(0x7e00),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x80402000000),
        BitBoard(0x4000000),
        BitBoard(0x20000),
        BitBoard(0x100),
        BitBoard(0x80),
        BitBoard(0x40),
        BitBoard(0xfe00),
        BitBoard(0x10101000000),
    ],
    [
        BitBoard(0x100804000000),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x100),
        BitBoard(0x0),
        BitBoard(0x1fe00),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x1008000000),
        BitBoard(0x4010000000),
        BitBoard(0x7f80000),
        BitBoard(0x404),
        BitBoard(0x201),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x2010000000),
        BitBoard(0x0),
        BitBoard(0x7f00000),
        BitBoard(0x0),
        BitBoard(0x402),
        BitBoard(0x0),
        BitBoard(0x40000),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x4020000000),
        BitBoard(0x10040000000),
        BitBoard(0x7e00000),
        BitBoard(0x1010),
        BitBoard(0x804),
        BitBoard(0x401),
        BitBoard(0xc0000),
        BitBoard(0x1010000000),
    ],
    [
        BitBoard(0x8040000000),
        BitBoard(0x0),
        BitBoard(0x7c00000),
        BitBoard(0x0),
        BitBoard(0x1008),
        BitBoard(0x0),
        BitBoard(0x1c0000),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x10080000000),
        BitBoard(0x40100000000),
        BitBoard(0x7800000),
        BitBoard(0x4040),
        BitBoard(0x2010),
        BitBoard(0x1004),
        BitBoard(0x3c0000),
        BitBoard(0x4040000000),
    ],
    [
        BitBoard(0x20100000000),
        BitBoard(0x0),
        BitBoard(0x7000000),
        BitBoard(0x0),
        BitBoard(0x4020),
        BitBoard(0x0),
        BitBoard(0x7c0000),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x40200000000),
        BitBoard(0x100400000000),
        BitBoard(0x6000000),
        BitBoard(0x10100),
        BitBoard(0x8040),
        BitBoard(0x4010),
        BitBoard(0xfc0000),
        BitBoard(0x10100000000),
    ],
    [
        BitBoard(0x80400000000),
        BitBoard(0x0),
        BitBoard(0x4000000),
        BitBoard(0x0),
        BitBoard(0x10080),
        BitBoard(0x0),
        BitBoard(0x1fc0000),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x100800000000),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x20100),
        BitBoard(0x10040),
        BitBoard(0x3fc0000),
        BitBoard(0x40400000000),
    ],
    [
        BitBoard(0x1000000000),
        BitBoard(0x0),
        BitBoard(0xff0000000),
        BitBoard(0x0),
        BitBoard(0x40201),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x2000000000),
        BitBoard(0x4000000000),
        BitBoard(0xfe0000000),
        BitBoard(0x101010),
        BitBoard(0x80402),
        BitBoard(0x40000),
        BitBoard(0x8000000),
        BitBoard(0x1000000000),
    ],
    [
        BitBoard(0x4000000000),
        BitBoard(0x0),
        BitBoard(0xfc0000000),
        BitBoard(0x0),
        BitBoard(0x100804),
        BitBoard(0x0),
        BitBoard(0x18000000),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x8000000000),
        BitBoard(0x10000000000),
        BitBoard(0xf80000000),
        BitBoard(0x404040),
        BitBoard(0x201008),
        BitBoard(0x100401),
        BitBoard(0x38000000),
        BitBoard(0x4000000000),
    ],
    [
        BitBoard(0x10000000000),
        BitBoard(0x0),
        BitBoard(0xf00000000),
        BitBoard(0x0),
        BitBoard(0x402010),
        BitBoard(0x0),
        BitBoard(0x78000000),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x20000000000),
        BitBoard(0x40000000000),
        BitBoard(0xe00000000),
        BitBoard(0x1010100),
        BitBoard(0x804020),
        BitBoard(0x401004),
        BitBoard(0xf8000000),
        BitBoard(0x10000000000),
    ],
    [
        BitBoard(0x40000000000),
        BitBoard(0x0),
        BitBoard(0xc00000000),
        BitBoard(0x0),
        BitBoard(0x1008040),
        BitBoard(0x0),
        BitBoard(0x1f8000000),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x80000000000),
        BitBoard(0x100000000000),
        BitBoard(0x800000000),
        BitBoard(0x4000000),
        BitBoard(0x2010080),
        BitBoard(0x1004010),
        BitBoard(0x3f8000000),
        BitBoard(0x40000000000),
    ],
    [
        BitBoard(0x100000000000),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x4020100),
        BitBoard(0x0),
        BitBoard(0x7f8000000),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x1fe000000000),
        BitBoard(0x10101010),
        BitBoard(0x8040201),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x1fc000000000),
        BitBoard(0x0),
        BitBoard(0x10080402),
        BitBoard(0x0),
        BitBoard(0x1000000000),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x1f8000000000),
        BitBoard(0x40404040),
        BitBoard(0x0),
        BitBoard(0x10040000),
        BitBoard(0x3000000000),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x1f0000000000),
        BitBoard(0x0),
        BitBoard(0x40201008),
        BitBoard(0x0),
        BitBoard(0x7000000000),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x1e0000000000),
        BitBoard(0x101010100),
        BitBoard(0x80402010),
        BitBoard(0x40100401),
        BitBoard(0xf000000000),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x1c0000000000),
        BitBoard(0x0),
        BitBoard(0x100804020),
        BitBoard(0x0),
        BitBoard(0x1f000000000),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x180000000000),
        BitBoard(0x404000000),
        BitBoard(0x201008040),
        BitBoard(0x100401004),
        BitBoard(0x3f000000000),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x100000000000),
        BitBoard(0x0),
        BitBoard(0x402010080),
        BitBoard(0x0),
        BitBoard(0x7f000000000),
        BitBoard(0x0),
    ],
    [
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x0),
        BitBoard(0x804020100),
        BitBoard(0x401004010),
        BitBoard(0xff000000000),
        BitBoard(0x0),
    ],
];

/// Bitmask of starting board positions for the black pieces
pub const BB_BLACK: BitBoard = BitBoard(0x1ffffa940000);

/// Bitmask of starting board positions for the white pieces
pub const BB_WHITE: BitBoard = BitBoard(0x52bffff);
