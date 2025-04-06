use std::fmt;

use regex::Regex;

use crate::bitboard::{BitBoard, BB_BLACK, BB_EMPTY, BB_POS, BB_WHITE, COLS, ROWS};
use crate::FanoronaError;
use crate::{CaptureType, Direction, Piece, Square};

/// Enum of error reasons for `is_capture`
#[derive(Debug, PartialEq)]
pub enum IsCaptureReason {
    FromEmpty,
    SquareOutOfBounds,
    CaptureOutOfBounds,
    CaptureEmpty,
    SelfPieceOnCaptureSquare,
    AmbiguousCapture,
    EndTurnMove,
}

impl std::error::Error for IsCaptureReason {}

impl fmt::Display for IsCaptureReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IsCaptureReason::FromEmpty => write!(f, "{}", "From square is empty"),
            IsCaptureReason::SquareOutOfBounds => write!(f, "{}", "To square is out of bounds"),
            IsCaptureReason::CaptureOutOfBounds => {
                write!(f, "{}", "Capture square is out of bounds")
            }
            IsCaptureReason::CaptureEmpty => {
                write!(f, "{}", "Capture square is empty (paika move)")
            }
            IsCaptureReason::SelfPieceOnCaptureSquare => {
                write!(f, "{}", "Own piece on capture square")
            }
            IsCaptureReason::AmbiguousCapture => write!(
                f,
                "{}",
                "Approach and withdrawal captures possible without clarification"
            ),
            IsCaptureReason::EndTurnMove => write!(f, "{}", "End turn move cannot be a capture"),
        }
    }
}

/// Representation of a Fanorona board position
///
/// Uses two bitboards to store the positions of the white and black pieces.
#[derive(Clone, Copy)]
pub struct BaseBoard {
    pieces: [BitBoard; 2],
}

impl fmt::Debug for BaseBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_chars = [['.'; COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                if self.pieces[Piece::White] & BB_POS[row * COLS + col] > 0 {
                    board_chars[row][col] = 'W';
                } else if self.pieces[Piece::Black] & BB_POS[row * COLS + col] > 0 {
                    board_chars[row][col] = 'B';
                }
            }
        }
        write!(
            f,
            "{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
            board_chars[4], board_chars[3], board_chars[2], board_chars[1], board_chars[0]
        )
    }
}

impl fmt::Display for BaseBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bb_str = String::new();
        let mut spaces: u32 = 0;
        for row in 0..ROWS {
            for col in 0..COLS {
                let white_bit = self.pieces[Piece::White] >> (row * COLS + col) & 1;
                let black_bit = self.pieces[Piece::Black] >> (row * COLS + col) & 1;
                if white_bit | black_bit == 1 {
                    if spaces > 0 {
                        let spaces_char = char::from_digit(spaces, 10).ok_or_else(|| fmt::Error)?;
                        bb_str.push(spaces_char);
                    }
                    spaces = 0;
                    if white_bit == 1 {
                        bb_str.push('W');
                    } else if black_bit == 1 {
                        bb_str.push('B');
                    }
                } else {
                    spaces += 1;
                }
            }
            if spaces > 0 {
                let spaces_char = char::from_digit(spaces, 10).ok_or_else(|| fmt::Error)?;
                bb_str.push(spaces_char);
            }
            spaces = 0;
            bb_str.push('/');
        }
        bb_str.pop(); // remove trailing '/'
        write!(f, "{}", bb_str)
    }
}

impl TryFrom<&str> for BaseBoard {
    type Error = FanoronaError;

    /// Parse a Fanorona board string into a `BaseBoard`
    ///
    /// A Fanorona board string is an adaptation of [FEN](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)
    /// with a few key simplifications -
    /// + The only piece types are white (`W`) and black (`B`)
    /// + There are only 5 ranks, but 9 files
    /// + The ordering of ranks is reversed, starting from rank 1 and going to rank 5
    fn try_from(board_str: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(
            r"(?x)
            ^(?P<r0>[WwBb1-9]+)/
            (?P<r1>[WwBb1-9]+)/
            (?P<r2>[WwBb1-9]+)/
            (?P<r3>[WwBb1-9]+)/
            (?P<r4>[WwBb1-9]+)
            ",
        )?;
        let caps = re.captures(board_str).ok_or_else(|| {
            FanoronaError::TryFromStrError(String::from(
                "BaseBoard regex did not capture any groups",
            ))
        })?;
        let mut rows = vec![String::new(); ROWS];
        for row in 0..ROWS {
            rows[row] = caps
                .name(format!("r{}", row).as_str())
                .ok_or_else(|| {
                    FanoronaError::TryFromStrError(format!(
                        "Row {} group was not captured",
                        row + 1
                    ))
                })?
                .as_str()
                .to_string();
        }
        let mut base_board = BaseBoard {
            pieces: [BB_EMPTY, BB_EMPTY],
        };
        for row in 0..ROWS {
            let mut col: u32 = 0;
            for c in rows[row].chars() {
                match c {
                    'w' | 'W' => {
                        let piece = Piece::White;
                        let at = Square::from((row, col as usize));
                        base_board.set_piece_at(piece, at);
                        col += 1;
                    }
                    'b' | 'B' => {
                        let piece = Piece::Black;
                        let at = Square::from((row, col as usize));
                        base_board.set_piece_at(piece, at);
                        col += 1;
                    }
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        col += char::to_digit(c, 10).ok_or_else(|| {
                            FanoronaError::TryFromStrError(String::from(
                                "could not parse number as valid sequence of contiguous empty spaces",
                        ))})?;
                    }
                    _ => {
                        return Err(FanoronaError::TryFromStrError(String::from(
                            "got invalid character in board string",
                        )));
                    }
                }
            }
        }
        Ok(base_board)
    }
}

impl PartialEq for BaseBoard {
    fn eq(&self, other: &Self) -> bool {
        self.pieces[0] == other.pieces[0] && self.pieces[1] == other.pieces[1]
    }
}

impl BaseBoard {
    pub fn new() -> BaseBoard {
        BaseBoard {
            pieces: [BB_BLACK, BB_WHITE],
        }
    }

    /// Returns the piece type located as a particular square
    pub fn piece_at(&self, at: Square) -> Option<Piece> {
        if self.pieces[Piece::White] & BB_POS[at] > 0 {
            Some(Piece::White)
        } else if self.pieces[Piece::Black] & BB_POS[at] > 0 {
            Some(Piece::Black)
        } else {
            None
        }
    }

    /// Remove a piece from a square on the board
    pub fn remove_piece_at(&mut self, at: Square) -> Option<Piece> {
        let piece = self.piece_at(at);
        self.pieces[Piece::Black] &= !BB_POS[at];
        self.pieces[Piece::White] &= !BB_POS[at];
        piece
    }

    /// Set a piece at the square on the board, overwriting what might have been there initially
    pub fn set_piece_at(&mut self, piece: Piece, at: Square) {
        self.pieces[piece] |= BB_POS[at]; // set piece of given color
        self.pieces[piece.other()] &= !BB_POS[at] // remove piece of other color
    }

    /// Operationalize a paika move that involves only moving a piece in a given direction, without any captures
    pub fn make_paika(&mut self, from: Square, direction: Direction) -> Result<(), FanoronaError> {
        let piece = self
            .remove_piece_at(from)
            .ok_or(FanoronaError::MoveError(String::from(format!(
                "Piece does not exist at {}",
                from.to_string()
            ))))?;
        let to = from
            .translate(direction)
            .ok_or(FanoronaError::MoveError(String::from(
                "Move was out of bounds",
            )))?;
        self.set_piece_at(piece, to);
        Ok(())
    }

    /// Test if a given move could result in an approach capture
    pub fn is_approach_capture(
        &self,
        from: Square,
        direction: Direction,
    ) -> Result<(), IsCaptureReason> {
        let moved_piece = self.piece_at(from).ok_or(IsCaptureReason::FromEmpty)?;
        let to = from
            .translate(direction)
            .ok_or(IsCaptureReason::SquareOutOfBounds)?;
        let approach_sq = to
            .translate(direction)
            .ok_or(IsCaptureReason::CaptureOutOfBounds)?;
        let piece = self
            .piece_at(approach_sq)
            .ok_or(IsCaptureReason::CaptureEmpty)?;
        if piece == moved_piece {
            Err(IsCaptureReason::SelfPieceOnCaptureSquare)
        } else {
            Ok(())
        }
    }

    /// Test if a given move could result in a withdraw capture
    pub fn is_withdraw_capture(
        &self,
        from: Square,
        direction: Direction,
    ) -> Result<(), IsCaptureReason> {
        let moved_piece = self.piece_at(from).ok_or(IsCaptureReason::FromEmpty)?;
        let withdraw_sq = from
            .translate(direction.mirror())
            .ok_or(IsCaptureReason::CaptureOutOfBounds)?;
        let piece = self
            .piece_at(withdraw_sq)
            .ok_or(IsCaptureReason::CaptureEmpty)?;
        if piece == moved_piece {
            Err(IsCaptureReason::SelfPieceOnCaptureSquare)
        } else {
            Ok(())
        }
    }

    /// Test if a given move is a capture, given an optional capture type
    ///
    /// If a capture type is provided, this function tests if the given move can be treated as a capturing move of that
    /// type. If no capture type is provided, it tries to treat the move as both an approach capture and a withdrawal
    /// capture. If both are possible, it raises an error, since the capture type must be unambiguous.
    pub fn is_capture(
        &self,
        from: Square,
        direction: Direction,
        capture_type: Option<CaptureType>,
    ) -> Result<(), IsCaptureReason> {
        match capture_type {
            Some(CaptureType::Approach) => self.is_approach_capture(from, direction),
            Some(CaptureType::Withdrawal) => self.is_withdraw_capture(from, direction),
            None => {
                let approach = self.is_approach_capture(from, direction);
                let withdraw = self.is_withdraw_capture(from, direction);
                if approach.is_ok() && withdraw.is_ok() {
                    Err(IsCaptureReason::AmbiguousCapture)
                } else if approach.is_ok() {
                    approach
                } else if withdraw.is_ok() {
                    withdraw
                } else {
                    approach // return approach result if neither is possible
                }
            }
        }
    }

    /// Check if a legal capturing move exists for the given side
    pub fn capture_exists(&self, side: Piece) -> bool {
        for square in Square::from(0) {
            match self.piece_at(square) {
                Some(piece) => {
                    if piece == side {
                        for direction in Direction::North {
                            for capture_type in vec![CaptureType::Approach, CaptureType::Withdrawal]
                            {
                                if self
                                    .is_capture(square, direction, Some(capture_type))
                                    .is_ok()
                                {
                                    return true;
                                }
                            }
                        }
                    }
                }
                None => {}
            }
        }
        return false;
    }

    /// Given a legal capture move, execute it on the board
    pub fn make_capture(
        &mut self,
        from: Square,
        direction: Direction,
        capture_type: Option<CaptureType>,
    ) -> Result<(), FanoronaError> {
        let moved_piece = self
            .piece_at(from)
            .ok_or(FanoronaError::MoveError(String::from(format!(
                "From square {} is empty",
                from.to_string()
            ))))?;
        self.make_paika(from, direction)?;

        let mut opp_pieces = self.pieces[moved_piece.other()];
        let capture_mask = match capture_type {
            Some(CaptureType::Approach) => {
                let ray_start =
                    from.translate(direction)
                        .ok_or(FanoronaError::MoveError(String::from(format!(
                            "Move is out of bounds"
                        ))))?;
                BitBoard::get_capture_mask(opp_pieces, ray_start, direction)
            }
            Some(CaptureType::Withdrawal) => {
                BitBoard::get_capture_mask(opp_pieces, from, direction.mirror())
            }
            None => {
                let ray_start =
                    from.translate(direction)
                        .ok_or(FanoronaError::MoveError(String::from(format!(
                            "Move is out of bounds"
                        ))))?;
                BitBoard::get_capture_mask(opp_pieces, ray_start, direction)
                    | BitBoard::get_capture_mask(opp_pieces, from, direction.mirror())
            }
        };
        opp_pieces &= !capture_mask;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::bitboard::{BB_BLACK, BB_EMPTY, BB_WHITE};

    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(
            "WWWWWWWWW/WWWWWWWWW/BWBW1BWBW/BBBBBBBBB/BBBBBBBBB",
            BaseBoard::new().to_string()
        );
        assert_eq!(
            "9/9/9/9/9",
            BaseBoard {
                pieces: [BB_EMPTY, BB_EMPTY]
            }
            .to_string()
        );
    }

    #[test]
    fn test_try_from() {
        assert_eq!(
            BaseBoard {
                pieces: [BB_EMPTY, BB_EMPTY]
            },
            BaseBoard::try_from("9/9/9/9/9").unwrap()
        );
        assert_eq!(
            BaseBoard {
                pieces: [BB_BLACK, BB_WHITE]
            },
            BaseBoard::try_from("WWWWWWWWW/WWWWWWWWW/BWBW1BWBW/BBBBBBBBB/BBBBBBBBB").unwrap()
        );
    }

    #[test]
    fn test_new() {
        assert_eq!(
            BaseBoard {
                pieces: [BB_BLACK, BB_WHITE]
            },
            BaseBoard::new()
        );
    }

    #[test]
    fn test_piece_at() {
        assert_eq!(
            BaseBoard::new().piece_at(Square::from((0, 0))),
            Some(Piece::White)
        );
    }

    #[test]
    fn test_remove_piece_at() {
        let mut board: BaseBoard = BaseBoard::new();
        assert!(board.remove_piece_at(Square::from(0)).is_some());
        assert!(board.piece_at(Square::from(0)).is_none());
    }

    #[test]
    fn test_set_piece_at() {
        let mut board: BaseBoard = BaseBoard::new();
        board.set_piece_at(Piece::Black, Square::from(0));
        assert_eq!(board.piece_at(Square::from(0)).unwrap(), Piece::Black);
    }

    #[test]
    fn test_make_paika() {
        let mut board: BaseBoard = BaseBoard::try_from("W8/9/9/9/9").unwrap();
        assert!(board.make_paika(Square::from(0), Direction::North).is_ok());
        assert_eq!(board.piece_at(Square::from((1, 0))), Some(Piece::White));
        assert_eq!(board.piece_at(Square::from((0, 0))), None);
    }

    #[test]
    fn test_is_approach_capture() {
        assert!(BaseBoard::new()
            .is_approach_capture(Square::from((1, 4)), Direction::North)
            .is_ok());
    }

    #[test]
    fn test_is_withdraw_capture() {
        assert!(BaseBoard::new()
            .is_withdraw_capture(Square::from((2, 3)), Direction::East)
            .is_ok());
    }

    #[test]
    fn test_is_capture() {
        assert!(BaseBoard::new()
            .is_capture(
                Square::from((1, 4)),
                Direction::North,
                Some(CaptureType::Approach)
            )
            .is_ok());
    }

    #[test]
    fn test_capture_exists() {
        assert!(BaseBoard::new().capture_exists(Piece::White));
    }

    #[test]
    fn test_make_capture() {
        let mut board: BaseBoard = BaseBoard::new();
        assert!(board
            .make_capture(
                Square::from((1, 4)),
                Direction::North,
                Some(CaptureType::Approach)
            )
            .is_ok());
    }
}
