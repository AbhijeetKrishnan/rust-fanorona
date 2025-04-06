#![warn(missing_docs)]

extern crate regex;

#[macro_use]
extern crate impl_ops;

mod bitboard;

mod base_board;
use base_board::BaseBoard;

mod r#move;
use r#move::Move;

mod square;
use square::Square;

mod direction;
use direction::Direction;

mod capture_type;
use capture_type::CaptureType;

mod piece;
use piece::Piece;

mod board;
pub use board::Board;

use std::fmt;

#[derive(Debug)]
pub enum FanoronaError {
    TryFromStrError(String),
    MoveError(String),
    SquareOutOfBoundsError(String),
    RegexError(regex::Error),
    ParseError(String),
}

impl std::error::Error for FanoronaError {}

impl From<regex::Error> for FanoronaError {
    fn from(err: regex::Error) -> FanoronaError {
        FanoronaError::RegexError(err)
    }
}

impl fmt::Display for FanoronaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FanoronaError::TryFromStrError(msg)
            | FanoronaError::MoveError(msg)
            | FanoronaError::SquareOutOfBoundsError(msg)
            | FanoronaError::ParseError(msg) => write!(f, "{}", msg),
            FanoronaError::RegexError(err) => write!(f, "{}", err.to_string()),
        }
    }
}
