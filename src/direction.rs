use crate::FanoronaError;
use std::{fmt, string::String};

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Into<usize> for Direction {
    fn into(self) -> usize {
        self.idx()
    }
}

impl TryFrom<&str> for Direction {
    type Error = FanoronaError;

    fn try_from(dir_str: &str) -> Result<Direction, FanoronaError> {
        match dir_str {
            "N" | "n" => Ok(Direction::North),
            "S" | "s" => Ok(Direction::South),
            "E" | "e" => Ok(Direction::East),
            "W" | "w" => Ok(Direction::West),
            "NW" | "nw" | "nW" | "Nw" => Ok(Direction::NorthWest),
            "NE" | "ne" | "nE" | "Ne" => Ok(Direction::NorthEast),
            "SW" | "sw" | "sW" | "Sw" => Ok(Direction::SouthWest),
            "SE" | "se" | "sE" | "Se" => Ok(Direction::SouthEast),
            _ => Err(FanoronaError::TryFromStrError(String::from(format!(
                "could not parse {} as direction",
                dir_str
            )))),
        }
    }
}

impl Direction {
    pub const fn idx(&self) -> usize {
        match self {
            Direction::North => 0usize,
            Direction::NorthEast => 1usize,
            Direction::East => 2usize,
            Direction::SouthEast => 3usize,
            Direction::South => 4usize,
            Direction::SouthWest => 5usize,
            Direction::West => 6usize,
            Direction::NorthWest => 7usize,
        }
    }

    pub const fn mirror(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::NorthEast => Direction::SouthWest,
            Direction::East => Direction::West,
            Direction::SouthEast => Direction::NorthWest,
            Direction::South => Direction::North,
            Direction::SouthWest => Direction::NorthEast,
            Direction::West => Direction::East,
            Direction::NorthWest => Direction::SouthEast,
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
