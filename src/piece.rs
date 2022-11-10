use std::ops::{Index, IndexMut};
use crate::bitboard::BitBoard;

#[derive(Debug, Clone, Copy)]
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