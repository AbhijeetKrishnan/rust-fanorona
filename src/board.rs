use std::fmt::{self, write};

use crate::{
    base_board::IsCaptureReason, bitboard, capture_type::CaptureType, direction::Direction,
    square::Square, BaseBoard, FanoronaError, Move, Piece,
};

#[derive(Debug)]
pub enum IsLegalReason {
    OwnPieceNotMoved,
    PieceMovingOutOfBounds,
    LastCaptureOutOfBounds,
    MoveMustFollowLastCapture,
    VisitingVisitedSquare,
    CannotCaptureSameDirection,
    PaikaWhenCaptureExists,
    AmbiguousCapture,
    EndTurnWithoutCaptureSequence,
    LastCaptureNotByCurrentPlayer,
    LastCaptureNoneInCaptureSequence,
    MovingToOccupiedSquare,
}

impl std::error::Error for IsLegalReason {}

impl fmt::Display for IsLegalReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IsLegalReason::OwnPieceNotMoved => write!(f, "{}", "The move must operate on the piece belonging to the player whose turn it is to play"),
            IsLegalReason::PieceMovingOutOfBounds => write!(f, "{}", "The move must not move the piece out of bounds"),
            IsLegalReason::LastCaptureOutOfBounds => write!(f, "{}", "The last capture was out of bounds"),
            IsLegalReason::MoveMustFollowLastCapture => write!(f, "{}", "If in a capturing sequence, the same piece must continue to be moved"),
            IsLegalReason::VisitingVisitedSquare => write!(f, "{}", "Cannot visit a previously visited square during a capturing sequence"),
            IsLegalReason::CannotCaptureSameDirection => write!(f, "{}", "Cannot capture in the same direction (approach or withdrawal) as the last capture"),
            IsLegalReason::PaikaWhenCaptureExists => write!(f, "{}", "Cannot play a paika move when a capture exists"),
            IsLegalReason::AmbiguousCapture => write!(f, "{}", "Must provide a capture type if it is ambiguous"),
            IsLegalReason::EndTurnWithoutCaptureSequence => write!(f, "{}", "Cannot play end turn when not in a capturing sequence"),
            IsLegalReason::LastCaptureNotByCurrentPlayer => write!(f, "{}", "Last capture was not made by current player, so end turn cannot be played by the current player"),
            IsLegalReason::LastCaptureNoneInCaptureSequence => write!(f, "{}", "Last capture cannot be None in a capturing sequence"),
            IsLegalReason::MovingToOccupiedSquare => write!(f, "{}", "Piece must move to an empty square"),
        }
    }
}

/// Reprentation of a Fanorona game state
#[derive(Debug, Clone, Copy)]
pub struct Board {
    /// the current board position
    base_board: BaseBoard,

    /// whose turn it is
    turn: Piece,

    /// a bitmask of all positions visited in the current capturing sequence (if in one)
    visited: bitboard::BitBoard,

    /// the last capturing move made (if any)
    last_capture: Option<Move>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.last_capture {
            Some(last_capture) => write!(
                f,
                "{} {} {} {}",
                self.base_board.to_string(),
                self.turn,
                self.visited
                    .as_squares()
                    .iter()
                    .map(|&sq| sq.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                last_capture,
            ),
            None => write!(
                f,
                "{} {} {} {}",
                self.base_board.to_string(),
                self.turn,
                self.visited
                    .as_squares()
                    .iter()
                    .map(|&sq| sq.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
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

    /// Pass turn to the other player
    fn pass_turn(&mut self) {
        self.turn = self.turn.other()
    }

    /// Test if a given move is a capturing move
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

    /// Execute a given move in the current game state
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
                if self.is_capture(fmove).is_ok() {
                    self.last_capture = Some(fmove);
                    self.visited |= bitboard::BitBoard::pos(from);
                    self.base_board.make_capture(from, direction, capture_type)
                } else {
                    self.last_capture = None;
                    self.visited &= bitboard::BB_EMPTY;
                    self.base_board.make_paika(from, direction)
                }
            }
        }
    }

    /// Test if the game is currently in a capturing sequence
    ///
    /// Implemented by testing if the `last_capture` field contains a valid move. This is only true if the last move
    /// was a valid capture.
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

    /// Test if a move is legal to play in the current game state
    pub fn is_legal(&self, fmove: Move) -> Result<(), IsLegalReason> {
        match fmove {
            Move::Move {
                from,
                direction,
                capture_type,
            } => {
                if self.base_board.piece_at(from) != Some(self.turn) {
                    return Err(IsLegalReason::OwnPieceNotMoved);
                }

                let to = from
                    .translate(direction)
                    .ok_or(IsLegalReason::PieceMovingOutOfBounds)?;

                if self.base_board.piece_at(to) != None {
                    return Err(IsLegalReason::MovingToOccupiedSquare);
                }

                // if `last_capture` is set (i.e., in a capturing sequence), then ensure that -
                // + the piece being moved is the one that was last moved
                // + the piece being moved is moving in the same or mirrored direction as the last capture
                if let Some(Move::Move {
                    from: lc_from,
                    direction: lc_dir,
                    capture_type: _,
                }) = self.last_capture
                {
                    let lc_to = lc_from
                        .translate(lc_dir)
                        .ok_or(IsLegalReason::LastCaptureOutOfBounds)?;
                    if from != lc_to {
                        return Err(IsLegalReason::MoveMustFollowLastCapture);
                    }
                    if bitboard::BitBoard::pos(to) & self.visited > 0 {
                        return Err(IsLegalReason::VisitingVisitedSquare);
                    }
                    if direction == lc_dir || direction == lc_dir.mirror() {
                        return Err(IsLegalReason::CannotCaptureSameDirection);
                    }
                    Ok(())
                // if the game state is not in a capturing sequence and a paika is played, there must not be any
                // available capture to play
                } else if !self
                    .base_board
                    .is_capture(from, direction, capture_type)
                    .is_ok()
                {
                    if self.base_board.capture_exists(self.turn) {
                        return Err(IsLegalReason::PaikaWhenCaptureExists);
                    }
                    Ok(())
                // if a capture is played but a capture type is not provided, the capture type must be unambiguous
                } else if self.is_capture(fmove) == Err(IsCaptureReason::AmbiguousCapture) {
                    return Err(IsLegalReason::AmbiguousCapture);
                } else {
                    Ok(())
                }
            }
            Move::EndTurn => {
                // end turn is only valid in a capturing sequence
                if !self.in_capture_seq() {
                    return Err(IsLegalReason::EndTurnWithoutCaptureSequence);
                }

                // end turn must be played for the current player - checked by verifying that the last capture move
                // moved a piece belonging to the current player
                if let Some(Move::Move {
                    from: lc_from,
                    direction: lc_dir,
                    capture_type: _,
                }) = self.last_capture
                {
                    let lc_to = lc_from
                        .translate(lc_dir)
                        .ok_or(IsLegalReason::LastCaptureOutOfBounds)?;
                    if self.base_board.piece_at(lc_to) != Some(self.turn) {
                        return Err(IsLegalReason::LastCaptureNotByCurrentPlayer);
                    }
                    Ok(())
                } else {
                    Err(IsLegalReason::LastCaptureNoneInCaptureSequence)
                }
            }
        }
    }

    /// A convenience function to execute a move in the current game state given its representation as a move string
    pub fn push_str(&mut self, fmove_str: &'static str) -> Result<(), FanoronaError> {
        let fmove = Move::try_from(fmove_str)?;
        println!("{:?}", fmove);
        self.push(fmove)
    }

    /// Return the list of all legal moves in the current game state
    pub fn legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for from in Square::new(0).iter() {
            if self.base_board.piece_at(*from) == Some(self.turn) {
                for direction in Direction::North {
                    for capture_type in vec![CaptureType::Approach, CaptureType::Withdrawal] {
                        let move_ = Move::Move {
                            from: *from,
                            direction,
                            capture_type: Some(capture_type),
                        };
                        if self.is_legal(move_).is_ok() {
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
    use rand::prelude::SliceRandom;

    use super::*;

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
    fn test_push_str() {
        todo!()
    }

    #[test]
    fn test_is_capture() {
        todo!()
    }

    #[test]
    fn test_is_legal() {
        let board = Board::new();
        assert!(board
            .is_legal(Move::Move {
                from: Square::try_from("e2").expect("Failed to create move e2"),
                direction: Direction::North,
                capture_type: Some(CaptureType::Approach),
            })
            .is_ok());
    }

    // TODO: fix failing test
    #[test]
    fn test_legal_moves() {
        let board = Board::new();
        let legal_moves = board.legal_moves();
        assert_eq!(legal_moves.len(), 4);
        assert!(legal_moves.contains(&Move::try_from("D2NEA").unwrap()));
        assert!(legal_moves.contains(&Move::try_from("D3EA").unwrap()));
        assert!(legal_moves.contains(&Move::try_from("E2NA").unwrap()));
        assert!(legal_moves.contains(&Move::try_from("F2NWA").unwrap()));
    }

    #[test]
    fn test_stress() {
        let times = 20;
        for _ in 1..times {
            let mut board = Board::new();
            loop {
                println!("{}", board);
                let legal_moves = board.legal_moves();
                println!("{:?}", legal_moves);
                if legal_moves.is_empty() {
                    break;
                }
                let move_ = legal_moves
                    .choose(&mut rand::thread_rng())
                    .expect("Failed to choose a legal move");
                println!("{}", move_);
                let _ = board.push(*move_).expect("Failed to push move");
            }
        }
    }
}
