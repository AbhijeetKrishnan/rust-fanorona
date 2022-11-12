use std::fmt;

use crate::bitboard::{BitBoard, BB_BLACK, BB_POS, BB_WHITE, COLS, ROWS};
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

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl BaseBoard {
    pub fn new() -> BaseBoard {
        BaseBoard {
            pieces: [BB_BLACK, BB_WHITE],
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
