use std::fmt;

use crate::{BaseBoard, Move, MoveError, Piece, bitboard};

pub struct Board {
    base_board: BaseBoard,
    turn: Piece,
    visited: bitboard::BitBoard,
    move_stack: Vec<Move>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.base_board.to_string())
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            base_board: BaseBoard::new(),
            turn: Piece::White,
            visited: bitboard::BB_EMPTY,
            move_stack: vec![],
        }
    }

    fn pass_turn(&mut self) {
        self.turn = self.turn.other()
    }

    pub fn push(&mut self, fmove: Move) {
        match fmove {
            Move::EndTurn => self.pass_turn(),
            Move::Move { from, direction } => {
                self.base_board.make_capture(from, direction, None);
            }
            Move::Capture {
                from,
                direction,
                capture_type,
            } => {
                self.base_board
                    .make_capture(from, direction, Some(capture_type));
            }
        }
    }

    pub fn push_str(&mut self, fmove_str: &'static str) -> Result<(), MoveError> {
        let fmove = Move::try_from(fmove_str)?;
        println!("{:?}", fmove);
        Ok(self.push(fmove))
    }
}
