use std::fmt;
use std::str;

use crate::capture_type::CaptureTypeError;
use crate::direction::DirectionError;
use crate::square::SquareError;
use crate::{CaptureType, Direction, Square};

use regex::Regex;

#[derive(Debug)]
pub enum MoveError {
    TryFromStrError(String),
    RegexError(regex::Error),
}

impl std::error::Error for MoveError {}

impl From<regex::Error> for MoveError {
    fn from(err: regex::Error) -> MoveError {
        MoveError::RegexError(err)
    }
}

impl From<SquareError> for MoveError {
    fn from(err: SquareError) -> MoveError {
        MoveError::TryFromStrError(err.to_string())
    }
}

impl From<DirectionError> for MoveError {
    fn from(err: DirectionError) -> MoveError {
        MoveError::TryFromStrError(err.to_string())
    }
}

impl From<CaptureTypeError> for MoveError {
    fn from(err: CaptureTypeError) -> MoveError {
        MoveError::TryFromStrError(err.to_string())
    }
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveError::TryFromStrError(msg) => write!(f, "{}", msg),
            MoveError::RegexError(err) => write!(f, "{}", err.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum Move {
    Move {
        from: Square,
        direction: Direction,
    },
    Capture {
        from: Square,
        direction: Direction,
        capture_type: CaptureType,
    },
    EndTurn,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::Move { from, direction } => {
                write!(f, "{}{}", from.to_string(), direction.to_string(),)
            }
            Move::Capture {
                from,
                direction,
                capture_type,
            } => write!(
                f,
                "{}{}{}",
                from.to_string(),
                direction.to_string(),
                capture_type.to_string()
            ),
            Move::EndTurn => write!(f, "X"),
        }
    }
}

impl TryFrom<&str> for Move {
    type Error = MoveError;
    fn try_from(move_str: &str) -> Result<Move, MoveError> {
        let re = Regex::new(
            r"(?x)
            ^(?P<from>[a-iA-I][1-5])
            (?P<direction>n|s|e|w|nw|ne|sw|se|N|S|E|W|NW|NE|SW|SE)
            (?P<capture_type>[fbFB])?
            |
            ^(?P<end_turn>[Xx])
        ",
        )?;
        let caps = re.captures(move_str).ok_or_else(|| {
            MoveError::TryFromStrError(String::from("regex did not capture any groups"))
        })?;
        match caps.name("end_turn") {
            Some(_) => Ok(Move::EndTurn),
            None => {
                let from_str = caps
                    .name("from")
                    .ok_or_else(|| {
                        MoveError::TryFromStrError(String::from("from group was not captured"))
                    })?
                    .as_str();
                let from = Square::try_from(from_str)?;

                let dir_str = caps
                    .name("direction")
                    .ok_or_else(|| {
                        MoveError::TryFromStrError(String::from("direction group was not captured"))
                    })?
                    .as_str();
                let direction = Direction::try_from(dir_str)?;

                let capture_type_opt: Option<CaptureType>;
                match caps.name("capture_type") {
                    None => capture_type_opt = None,
                    Some(capture_type_m) => {
                        let capture_type_str = capture_type_m.as_str();
                        let capture_type_res = CaptureType::try_from(capture_type_str)?;
                        capture_type_opt = Some(capture_type_res);
                    }
                }

                match capture_type_opt {
                    Some(capture_type) => Ok(Move::Capture {
                        from,
                        direction,
                        capture_type,
                    }),
                    None => Ok(Move::Move { from, direction }),
                }
            }
        }
    }
}
