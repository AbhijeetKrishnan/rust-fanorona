use crate::bitboard::BitBoard;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Black,
    White,
}

impl Piece {
    pub fn other(&self) -> Piece {
        match self {
            Piece::Black => Piece::White,
            Piece::White => Piece::Black,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Piece::Black => write!(f, "B"),
            Piece::White => write!(f, "W"),
        }
    }
}

impl Index<Piece> for [BitBoard; 2] {
    type Output = BitBoard;

    fn index(&self, index: Piece) -> &Self::Output {
        match index {
            Piece::Black => &self[0],
            Piece::White => &self[1],
        }
    }
}

impl IndexMut<Piece> for [BitBoard; 2] {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        match index {
            Piece::Black => &mut self[0],
            Piece::White => &mut self[1],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bitboard::{BB_BLACK, BB_WHITE};

    use super::*;

    #[test]
    fn test_other_white() {
        assert_eq!(Piece::Black, Piece::White.other());
        assert_eq!(Piece::White, Piece::Black.other());
    }

    #[test]
    fn test_index() {
        let bb = [BB_BLACK, BB_WHITE];
        let piece = Piece::White;
        assert_eq!(BB_WHITE, bb[piece]);
        assert_eq!(BB_BLACK, bb[piece.other()]);
    }

    #[test]
    fn test_index_mut() {
        let mut bb = [BB_BLACK, BB_WHITE];
        let piece = Piece::Black;
        assert_eq!(BB_BLACK, bb[piece]);
        assert_eq!(BB_WHITE, bb[piece.other()]);
    }
}
