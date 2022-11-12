use std::fmt;

use crate::{bitboard, BaseBoard, FanoronaError, Move, Piece};

#[derive(Debug, Clone, Copy)]
pub struct Board {
    base_board: BaseBoard,
    turn: Piece,
    visited: bitboard::BitBoard,
    last_capture: Option<Move>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.base_board.to_string())
    }
}

impl TryFrom<&str> for Board {
    type Error = FanoronaError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            base_board: BaseBoard::new(),
            turn: Piece::White,
            visited: bitboard::BB_EMPTY,
            last_capture: None,
        }
    }

    fn pass_turn(&mut self) {
        self.turn = self.turn.other()
    }

    fn in_capture_seq(&self) -> bool {
        if let Some(Move::Move {
            from: _,
            direction: _,
            capture_type: _,
        }) = self.last_capture
        {
            true
        } else {
            false
        }
    }

    pub fn push(&mut self, fmove: Move) {
        match fmove {
            Move::EndTurn => {
                self.visited &= bitboard::BB_EMPTY;
                self.last_capture = None;
                self.pass_turn();
            }
            Move::Move {
                from,
                direction,
                capture_type,
            } => {
                self.last_capture = Some(fmove);
                self.visited |= bitboard::BB_POS[from.idx()];
                self.base_board.make_capture(from, direction, capture_type);
            }
        }
    }

    pub fn is_capture(&self, fmove: Move) -> bool {
        todo!()
    }

    pub fn legal_move(&self, fmove: Move) -> bool {
        match fmove {
            Move::Move {
                from,
                direction,
                capture_type,
            } => {
                if let Some(Move::Move {
                    from: lc_from,
                    direction: lc_dir,
                    capture_type: _,
                }) = self.last_capture
                {
                    from == lc_from
                        && bitboard::BB_POS[from.translate(direction).idx()] & self.visited == 0
                        && direction != lc_dir
                } else if !self.base_board.is_capture(from, direction, capture_type) {
                    !self.base_board.capture_exists() // if paika is played, possible capture must not exist
                } else {
                    // if capture type is not provided, capture must be unambiguous
                    !(capture_type.is_none()
                        && self
                            .base_board
                            .is_capture(from, direction.mirror(), capture_type))
                }
            }
            Move::EndTurn => {
                // end turn is assumed to be played for current turn color
                // end turn is only valid in a capturing sequence
                self.in_capture_seq()
            }
        }
    }

    pub fn push_str(&mut self, fmove_str: &'static str) -> Result<(), FanoronaError> {
        let fmove = Move::try_from(fmove_str)?;
        println!("{:?}", fmove);
        Ok(self.push(fmove))
    }
}
