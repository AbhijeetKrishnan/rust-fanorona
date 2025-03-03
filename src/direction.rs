use crate::{bitboard::BitBoard, FanoronaError};
use std::{fmt, ops::Index, string::String};

/// A representation of the eight move directions in Fanorona
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
    X, // dummy direction to stop iteration
}

impl fmt::Display for Direction {
    /// Prints a direction in uppercase
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
            Direction::X => String::from("X"),
        };
        write!(f, "{}", dir_str)
    }
}

impl Into<usize> for Direction {
    fn into(self) -> usize {
        match self {
            Direction::North => 0usize,
            Direction::NorthEast => 1usize,
            Direction::East => 2usize,
            Direction::SouthEast => 3usize,
            Direction::South => 4usize,
            Direction::SouthWest => 5usize,
            Direction::West => 6usize,
            Direction::NorthWest => 7usize,
            Direction::X => 8usize,
        }
    }
}

impl TryFrom<&str> for Direction {
    type Error = FanoronaError;

    /// Parse a direction string as a Direction
    ///
    /// Parsing is case-insensitive
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

impl Index<Direction> for [BitBoard; 8] {
    type Output = BitBoard;

    fn index(&self, index: Direction) -> &Self::Output {
        &self[<Direction as Into<usize>>::into(index)]
    }
}

impl Iterator for Direction {
    type Item = Direction;

    /// Iterate clockwise over the directions and stop at NorthWest
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Direction::North => {
                *self = Direction::NorthEast;
                Some(Direction::North)
            }
            Direction::NorthEast => {
                *self = Direction::East;
                Some(Direction::NorthEast)
            }
            Direction::East => {
                *self = Direction::SouthEast;
                Some(Direction::East)
            }
            Direction::SouthEast => {
                *self = Direction::South;
                Some(Direction::SouthEast)
            }
            Direction::South => {
                *self = Direction::SouthWest;
                Some(Direction::South)
            }
            Direction::SouthWest => {
                *self = Direction::West;
                Some(Direction::SouthWest)
            }
            Direction::West => {
                *self = Direction::NorthWest;
                Some(Direction::West)
            }
            Direction::NorthWest => {
                *self = Direction::X;
                Some(Direction::NorthWest)
            }
            Direction::X => None,
        }
    }
}

impl Direction {
    /// Return the mirror image of the direction flipped 180°
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
            Direction::X => Direction::X,
        }
    }

    /// Return the delta needed to traverse the board in this direction in square-index space
    pub const fn to_increment(self) -> i8 {
        match self {
            Direction::North => 9,
            Direction::NorthEast => 10,
            Direction::East => 1,
            Direction::SouthEast => -8,
            Direction::South => -9,
            Direction::SouthWest => -10,
            Direction::West => -1,
            Direction::NorthWest => 8,
            Direction::X => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bitboard::BB_RAY;

    use super::*;

    #[test]
    fn test_display() {
        assert_eq!("N", Direction::North.to_string());
        assert_eq!("NE", Direction::NorthEast.to_string());
        assert_eq!("E", Direction::East.to_string());
        assert_eq!("SE", Direction::SouthEast.to_string());
        assert_eq!("S", Direction::South.to_string());
        assert_eq!("SW", Direction::SouthWest.to_string());
        assert_eq!("W", Direction::West.to_string());
        assert_eq!("NW", Direction::NorthWest.to_string());
    }

    #[test]
    fn test_try_from() {
        assert_eq!(Direction::North, Direction::try_from("N").unwrap());
        assert_eq!(Direction::SouthEast, Direction::try_from("sE").unwrap());
        assert_ne!(Direction::NorthWest, Direction::try_from("Sw").unwrap());
        assert!(Direction::try_from("Sww").is_err());
    }

    #[test]
    fn test_into() {
        assert_eq!(0usize, Direction::North.into());
        assert_eq!(1usize, Direction::NorthEast.into());
        assert_eq!(2usize, Direction::East.into());
        assert_eq!(3usize, Direction::SouthEast.into());
        assert_eq!(4usize, Direction::South.into());
        assert_eq!(5usize, Direction::SouthWest.into());
        assert_eq!(6usize, Direction::West.into());
        assert_eq!(7usize, Direction::NorthWest.into());
    }

    #[test]
    fn test_index() {
        assert_eq!(BB_RAY[0][0], BB_RAY[0][Direction::North]);
        assert_eq!(BB_RAY[0][1], BB_RAY[0][Direction::NorthEast]);
        assert_eq!(BB_RAY[0][2], BB_RAY[0][Direction::East]);
        assert_eq!(BB_RAY[0][3], BB_RAY[0][Direction::SouthEast]);
        assert_eq!(BB_RAY[0][4], BB_RAY[0][Direction::South]);
        assert_eq!(BB_RAY[0][5], BB_RAY[0][Direction::SouthWest]);
        assert_eq!(BB_RAY[0][6], BB_RAY[0][Direction::West]);
        assert_eq!(BB_RAY[0][7], BB_RAY[0][Direction::NorthWest]);
    }

    #[test]
    fn test_mirror() {
        assert_eq!(Direction::South, Direction::North.mirror());
        assert_eq!(Direction::SouthWest, Direction::NorthEast.mirror());
        assert_eq!(Direction::West, Direction::East.mirror());
        assert_eq!(Direction::NorthWest, Direction::SouthEast.mirror());
        assert_eq!(Direction::North, Direction::South.mirror());
        assert_eq!(Direction::NorthEast, Direction::SouthWest.mirror());
        assert_eq!(Direction::East, Direction::West.mirror());
        assert_eq!(Direction::SouthEast, Direction::NorthWest.mirror());
    }
}
