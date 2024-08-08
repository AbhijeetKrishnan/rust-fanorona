use std::fmt;

use crate::{
    base_board::IsCaptureReason, bitboard, capture_type, direction::Direction, square::Square,
    BaseBoard, FanoronaError, Move, Piece,
};

#[derive(Debug, Clone, Copy)]
pub struct Board {
    base_board: BaseBoard,
    turn: Piece,
    visited: bitboard::BitBoard,
    last_capture: Option<Move>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.last_capture {
            // TODO: handle printing of self.visited better
            Some(last_capture) => write!(
                f,
                "{} {} {} {}",
                self.base_board.to_string(),
                self.turn,
                self.visited,
                last_capture,
            ),
            None => write!(
                f,
                "{} {} {} {}",
                self.base_board.to_string(),
                self.turn,
                self.visited,
                "-"
            ),
        }
    }
}

impl TryFrom<&str> for Board {
    type Error = FanoronaError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut iter = value.split_whitespace();
        let board_str = iter.next().ok_or(FanoronaError::ParseError(String::from(
            "Could not get board string",
        )))?;
        let turn_str = iter.next().ok_or(FanoronaError::ParseError(String::from(
            "Could not get turn string",
        )))?;
        let visited_str = iter.next().ok_or(FanoronaError::ParseError(String::from(
            "Could not get visited string",
        )))?;
        let last_capture_str = iter.next().ok_or(FanoronaError::ParseError(String::from(
            "Could not get last capture string",
        )))?;

        let base_board = BaseBoard::try_from(board_str)?;
        let turn = Piece::try_from(turn_str)?;
        let visited = bitboard::BitBoard::try_from(visited_str)?;
        let last_capture = Move::try_from(last_capture_str)?;

        Ok(Board {
            base_board,
            turn,
            visited,
            last_capture: Some(last_capture),
        })
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

    pub fn push(&mut self, fmove: Move) -> Result<(), FanoronaError> {
        match fmove {
            Move::EndTurn => {
                self.visited &= bitboard::BB_EMPTY;
                self.last_capture = None;
                self.pass_turn();
                Ok(())
            }
            Move::Move {
                from,
                direction,
                capture_type,
            } => {
                self.last_capture = Some(fmove);
                self.visited |= bitboard::BB_POS[from];
                self.base_board.make_capture(from, direction, capture_type)
            }
        }
    }

    pub fn is_capture(&self, fmove: Move) -> Result<(), IsCaptureReason> {
        match fmove {
            Move::Move {
                from,
                direction,
                capture_type,
            } => self.base_board.is_capture(from, direction, capture_type),
            Move::EndTurn => Err(IsCaptureReason::EndTurnMove),
        }
    }

    pub fn is_legal(&self, fmove: Move) -> bool {
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
                    let to_opt = from.translate(direction);
                    if let Some(to) = to_opt {
                        from == lc_from
                            && bitboard::BB_POS[to] & self.visited == 0
                            && direction != lc_dir
                    } else {
                        false
                    }
                } else if !self
                    .base_board
                    .is_capture(from, direction, capture_type)
                    .is_ok()
                {
                    !self.base_board.capture_exists(self.turn) // if paika is played, possible capture must not exist
                } else {
                    // if capture type is not provided, capture must be unambiguous
                    !(capture_type.is_none()
                        && self
                            .base_board
                            .is_capture(from, direction.mirror(), capture_type)
                            .is_ok())
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
        self.push(fmove)
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for from in Square::new(0).iter() {
            if self.base_board.piece_at(*from) == Some(self.turn) {
                for direction in Direction::North {
                    for capture_type in capture_type::CaptureType::Approach {
                        let move_ = Move::Move {
                            from: *from,
                            direction,
                            capture_type: Some(capture_type),
                        };
                        if self.is_legal(move_) {
                            moves.push(move_);
                        }
                    }
                }
            }
        }
        if self.in_capture_seq() {
            moves.push(Move::EndTurn);
        }
        moves
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_display() {
        todo!()
    }

    #[test]
    fn test_try_from() {
        todo!()
    }

    #[test]
    fn test_new() {
        todo!()
    }

    #[test]
    fn test_pass_turn() {
        todo!()
    }

    #[test]
    fn test_in_capture_seq() {
        todo!()
    }

    #[test]
    fn test_push() {
        todo!()
    }

    #[test]
    fn test_is_capture() {
        todo!()
    }

    #[test]
    fn test_is_legal() {
        todo!()
    }

    #[test]
    fn test_push_str() {
        todo!()
    }

    #[test]
    fn test_legal_moves() {
        todo!()
    }
}
