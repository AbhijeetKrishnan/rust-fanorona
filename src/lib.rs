extern crate regex;

mod bitboard;
use bitboard::BaseBoard;

mod action;
use action::Move;
#[macro_use]
extern crate impl_ops;

use action::MoveError;

mod square;
use square::{Square, SquareIterator};

mod direction;
use direction::Direction;

mod capture_type;
use capture_type::CaptureType;

mod piece;
use piece::Piece;

mod board;
pub use board::Board;
