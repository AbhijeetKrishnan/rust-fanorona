use std::{fmt, string::String};

use crate::direction::Direction;
use crate::FanoronaError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square(usize);

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (row, col): (usize, usize) = (*self).into();
        let row_str = row.to_string();
        let col_str = match col {
            0usize => Ok('A'),
            1usize => Ok('B'),
            2usize => Ok('C'),
            3usize => Ok('D'),
            4usize => Ok('E'),
            5usize => Ok('F'),
            6usize => Ok('G'),
            7usize => Ok('H'),
            8usize => Ok('I'),
            _ => Err(fmt::Error),
        }?;
        write!(f, "{}{}", row_str, col_str)
    }
}

impl From<u32> for Square {
    fn from(sq: u32) -> Self {
        Square(sq as usize)
    }
}

impl From<usize> for Square {
    fn from(sq: usize) -> Self {
        Square(sq)
    }
}

impl From<(usize, usize)> for Square {
    fn from(move_tuple: (usize, usize)) -> Square {
        Square(move_tuple.0 * 9 + move_tuple.1)
    }
}

impl Into<usize> for Square {
    fn into(self) -> usize {
        self.0
    }
}

impl Into<(usize, usize)> for Square {
    fn into(self) -> (usize, usize) {
        (self.0 / 9, self.0 % 9)
    }
}

impl TryFrom<&str> for Square {
    type Error = FanoronaError;

    fn try_from(square_str: &str) -> Result<Square, FanoronaError> {
        let row = square_str
            .chars()
            .nth(1)
            .ok_or_else(|| FanoronaError::TryFromStrError(String::from("row char does not exist")))?
            .to_digit(10)
            .ok_or_else(|| {
                FanoronaError::TryFromStrError(String::from("could not convert row to number"))
            })? as usize;
        let col = match square_str.chars().nth(0).ok_or_else(|| {
            FanoronaError::TryFromStrError(String::from("col char does not exist"))
        })? {
            'a' | 'A' => Ok(0usize),
            'b' | 'B' => Ok(1usize),
            'c' | 'C' => Ok(2usize),
            'd' | 'D' => Ok(3usize),
            'e' | 'E' => Ok(4usize),
            'f' | 'F' => Ok(5usize),
            'g' | 'G' => Ok(6usize),
            'h' | 'H' => Ok(7usize),
            'i' | 'I' => Ok(8usize),
            _ => Err(FanoronaError::TryFromStrError(String::from(
                "could not convert col to number",
            ))),
        }?;
        Ok(Square::from((row, col)))
    }
}

impl Square {
    pub const fn new(sq: usize) -> Square {
        Square(sq)
    }

    #[inline]
    pub const fn idx(&self) -> usize {
        self.0
    }

    #[inline]
    pub const fn translate(self, direction: Direction) -> Square {
        match direction {
            Direction::North => Square(self.0 + 9),
            Direction::NorthEast => Square(self.0 + 10),
            Direction::East => Square(self.0 + 1),
            Direction::SouthEast => Square(self.0 - 8),
            Direction::South => Square(self.0 - 9),
            Direction::SouthWest => Square(self.0 - 10),
            Direction::West => Square(self.0 - 1),
            Direction::NorthWest => Square(self.0 + 8),
        }
    }
}

pub struct SquareIterator(usize);

impl SquareIterator {
    pub fn new(start: usize) -> Self {
        SquareIterator(start)
    }
}

impl Iterator for SquareIterator {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        let result: Option<Square>;
        if self.0 < 45 {
            result = Some(Square(self.0));
            self.0 += 1;
        } else {
            result = None;
        }
        result
    }
}
