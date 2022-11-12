use std::fmt;

use regex::Regex;

use crate::bitboard::{BitBoard, BB_EMPTY, BB_POS, COLS, ROWS};
use crate::FanoronaError;
use crate::{CaptureType, Direction, Piece, Square};

#[derive(Debug, Clone, Copy)]
pub struct BaseBoard {
    pieces: [BitBoard; 2],
}

impl fmt::Display for BaseBoard {
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

impl TryFrom<&str> for BaseBoard {
    type Error = FanoronaError;

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
        let mut base_board = BaseBoard::new();
        for row in 0..ROWS {
            let mut col: u32 = 0;
            for c in rows[row].chars() {
                match c {
                    'w' | 'W' => {
                        let piece = Piece::White;
                        let at = Square::from((row, col as usize));
                        base_board.set_piece_at(piece, at);
                    }
                    'b' | 'B' => {
                        let piece = Piece::Black;
                        let at = Square::from((row, col as usize));
                        base_board.set_piece_at(piece, at);
                    }
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        col += char::to_digit(c, 10).ok_or_else(|| FanoronaError::TryFromStrError(String::from("could not parse number as valid sequence of contiguous empty spaces")))?;
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

impl BaseBoard {
    pub fn new() -> BaseBoard {
        BaseBoard {
            pieces: [BB_EMPTY, BB_EMPTY],
        }
    }

    pub fn piece_at(&self, at: Square) -> Option<Piece> {
        if self.pieces[Piece::White] & !BB_POS[at.idx()] > 0 {
            Some(Piece::White)
        } else if self.pieces[Piece::Black] & !BB_POS[at.idx()] > 0 {
            Some(Piece::Black)
        } else {
            None
        }
    }

    pub fn remove_piece_from(&mut self, at: Square) -> Option<Piece> {
        let piece = self.piece_at(at);
        self.pieces[Piece::Black] &= !BB_POS[at.idx()];
        self.pieces[Piece::White] &= !BB_POS[at.idx()];
        piece
    }

    pub fn set_piece_at(&mut self, piece: Piece, at: Square) {
        self.pieces[piece] |= BB_POS[at.idx()]
    }

    pub fn make_paika(&mut self, from: Square, direction: Direction) {
        let piece = self.remove_piece_from(from);
        let to = from.translate(direction);
        match piece {
            Some(piece) => self.set_piece_at(piece, to),
            None => (),
        };
    }

    pub fn capture_exists(&self) -> bool {
        todo!()
    }

    pub fn is_capture(
        &self,
        from: Square,
        direction: Direction,
        capture_type: Option<CaptureType>,
    ) -> bool {
        todo!()
    }

    pub fn make_capture(
        &mut self,
        from: Square,
        direction: Direction,
        capture_type: Option<CaptureType>,
    ) {
        let moved_piece = self.piece_at(from).unwrap();
        self.make_paika(from, direction);

        let mut opp_pieces = self.pieces[moved_piece.other()];
        let capture_mask = match capture_type {
            Some(CaptureType::Approach) => {
                BitBoard::get_capture_mask(opp_pieces, from.translate(direction), direction)
            }
            Some(CaptureType::Withdrawal) => BitBoard::get_capture_mask(
                opp_pieces,
                from.translate(direction.mirror()),
                direction.mirror(),
            ),
            None => {
                BitBoard::get_capture_mask(opp_pieces, from.translate(direction), direction)
                    | BitBoard::get_capture_mask(
                        opp_pieces,
                        from.translate(direction.mirror()),
                        direction.mirror(),
                    )
            }
        };
        opp_pieces &= !capture_mask;
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        todo!()
    }

    #[test]
    fn test_try_from() {
        todo!()
    }

    fn test_new() {
        todo!()
    }

    fn test_piece_at() {
        todo!()
    }

    fn test_remove_piece_at() {
        todo!()
    }

    fn test_set_piece_at() {
        todo!()
    }

    fn test_make_paika() {
        todo!()
    }

    fn test_capture_exists() {
        todo!()
    }

    fn test_is_capture() {
        todo!()
    }

    fn test_make_capture() {
        todo!()
    }
}
