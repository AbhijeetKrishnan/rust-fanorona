use std::fmt;
use std::str;

use crate::{CaptureType, Direction, FanoronaError, Square};

use regex::Regex;

/// The representation of a Fanorona move
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    /// A move that moves a piece from a particular square, in a particular direction, and using an approach or
    /// withdrawal capture type (if applicable)
    Move {
        from: Square,
        direction: Direction,
        capture_type: Option<CaptureType>,
    },

    /// A move that ends the current turn if in a capture sequence
    EndTurn,
}

impl fmt::Display for Move {
    /// Print a move with its from square, direction and capture type (if any). End turn moves are printed as "X".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::Move {
                from,
                direction,
                capture_type: None,
            } => {
                write!(f, "{}{}", from.to_string(), direction.to_string(),)
            }
            Move::Move {
                from,
                direction,
                capture_type: Some(capture_type),
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
    type Error = FanoronaError;

    /// Parse a move string to create a Move
    ///
    /// A move string specifies from square, direction and capture type ("F" for forward, "B" for backward) if any.
    /// The end turn move is represented as "X".
    /// The parsing is case-insensitive.
    fn try_from(move_str: &str) -> Result<Move, FanoronaError> {
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
            FanoronaError::TryFromStrError(String::from("Move regex did not capture any groups"))
        })?;
        match caps.name("end_turn") {
            Some(_) => Ok(Move::EndTurn),
            None => {
                let from_str = caps
                    .name("from")
                    .ok_or_else(|| {
                        FanoronaError::TryFromStrError(String::from("from group was not captured"))
                    })?
                    .as_str();
                let from = Square::try_from(from_str)?;

                let dir_str = caps
                    .name("direction")
                    .ok_or_else(|| {
                        FanoronaError::TryFromStrError(String::from(
                            "direction group was not captured",
                        ))
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

                Ok(Move::Move {
                    from,
                    direction,
                    capture_type: capture_type_opt,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let action: Move = Move::Move {
            from: Square::new(0).unwrap(),
            direction: Direction::North,
            capture_type: None,
        };
        assert_eq!("A1N", format!("{}", action));

        let end_turn: Move = Move::EndTurn;
        assert_eq!("X", format!("{}", end_turn));
    }

    #[test]
    fn test_try_from() {
        let move_str = "A1N";
        let action = Move::Move {
            from: Square::new(0).unwrap(),
            direction: Direction::North,
            capture_type: None,
        };
        assert_eq!(action, Move::try_from(move_str).unwrap())
    }
}
