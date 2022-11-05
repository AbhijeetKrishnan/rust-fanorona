use std::{fmt, string::String};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

#[derive(Debug)]
pub enum DirectionError {
    TryFromStrError(String),
}

impl std::error::Error for DirectionError {}

impl fmt::Display for DirectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DirectionError::TryFromStrError(msg) => write!(f, "{}", msg),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dir_str = match self {
            Direction::North => String::from("N"),
            Direction::South => String::from("S"),
            Direction::East => String::from("E"),
            Direction::West => String::from("W"),
            Direction::NorthWest => String::from("NW"),
            Direction::NorthEast => String::from("NE"),
            Direction::SouthWest => String::from("SW"),
            Direction::SouthEast => String::from("SE"),
        };
        write!(f, "{}", dir_str)
    }
}

impl TryFrom<&str> for Direction {
    type Error = DirectionError;

    fn try_from(dir_str: &str) -> Result<Direction, DirectionError> {
        match dir_str {
            "N" | "n" => Ok(Direction::North),
            "S" | "s" => Ok(Direction::South),
            "E" | "e" => Ok(Direction::East),
            "W" | "w" => Ok(Direction::West),
            "NW" | "nw" | "nW" | "Nw" => Ok(Direction::NorthWest),
            "NE" | "ne" | "nE" | "Ne" => Ok(Direction::NorthEast),
            "SW" | "sw" | "sW" | "Sw" => Ok(Direction::SouthWest),
            "SE" | "se" | "sE" | "Se" => Ok(Direction::SouthEast),
            _ => Err(DirectionError::TryFromStrError(String::from(format!(
                "could not parse {} as direction",
                dir_str
            )))),
        }
    }
}

#[test]
fn test_try_from() {
    assert_eq!(Direction::North, Direction::try_from("N").unwrap());
    assert_eq!(Direction::SouthEast, Direction::try_from("sE").unwrap());
    assert_ne!(Direction::NorthWest, Direction::try_from("Sw").unwrap());
    assert!(Direction::try_from("Sww").is_err());
}
