use std::fmt;

use crate::{BaseBoard, Piece, Move, MoveError};


pub struct Board {
    base_board: BaseBoard,
    turn: Piece,
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
            turn: Piece::WHITE,
            move_stack: vec![],
        }
    }

    pub fn push(&mut self, fmove: Move) -> Result<(), MoveError> {
        todo!()
    }

    pub fn push_str(&mut self, fmove_str: &'static str) -> Result<(), MoveError> {
        let fmove = Move::try_from(fmove_str)?;
        println!("{:?}", fmove);
        self.push(fmove)
    }
}