use std::ops::Index;
use std::{fmt, string::String};

use crate::bitboard::{BitBoard, COLS, ROWS};
use crate::direction::Direction;
use crate::FanoronaError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square(usize);

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (row, col): (usize, usize) = (*self).into();
        let row_str = (row + 1).to_string();
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
        write!(f, "{}{}", col_str, row_str)
    }
}

impl From<usize> for Square {
    fn from(sq: usize) -> Self {
        Square(sq)
    }
}

impl From<(usize, usize)> for Square {
    fn from(move_tuple: (usize, usize)) -> Square {
        Square(move_tuple.0 * COLS + move_tuple.1)
    }
}

impl Into<(usize, usize)> for Square {
    fn into(self) -> (usize, usize) {
        (self.0 / COLS, self.0 % COLS)
    }
}

impl Index<Square> for [BitBoard; ROWS * COLS] {
    type Output = BitBoard;

    fn index(&self, index: Square) -> &Self::Output {
        &self[index.0]
    }
}

impl Index<Square> for [[BitBoard; 8]; ROWS * COLS] {
    type Output = [BitBoard; 8];

    fn index(&self, index: Square) -> &Self::Output {
        &self[index.0]
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
            })? as usize
            - 1;
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
    pub fn new(sq: usize) -> Result<Square, FanoronaError> {
        if sq < ROWS * COLS {
            Ok(Square(sq))
        } else {
            Err(FanoronaError::SquareOutOfBoundsError(String::from(
                "Square at {} is out of bounds",
            )))
        }
    }

    #[inline]
    pub const fn translate(self, direction: Direction) -> Option<Square> {
        let final_pos = (self.0 as i8) + direction.to_increment();
        if final_pos < 0 || (final_pos as usize) > ROWS * COLS {
            None
        } else {
            Some(Square(final_pos as usize))
        }
    }
}

pub struct SquareIterator(Square, Direction);

impl SquareIterator {
    pub fn new(start: Square, dir: Direction) -> Self {
        SquareIterator(start, dir)
    }
}

impl Iterator for SquareIterator {
    type Item = Square;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.0.translate(self.1);
        if let Some(square) = result {
            self.0 = square;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!("A1", Square(0).to_string());
        assert_eq!("I5", Square(44).to_string());
    }

    #[test]
    fn test_from_u32() {
        assert_eq!(Square(0), Square::from(0usize));
        assert_eq!(Square(33), Square::from(33usize));
        assert_eq!(Square(500), Square::from(500usize));
    }

    #[test]
    fn test_from_usize() {
        todo!()
    }

    #[test]
    fn test_from_tuple() {
        todo!()
    }

    #[test]
    fn test_into_tuple() {
        todo!()
    }

    #[test]
    fn test_try_from() {
        assert_eq!(Square(0), Square::try_from("A1").unwrap());
        assert!(Square::try_from("X1").is_err());
    }

    #[test]
    fn test_new() {
        todo!()
    }

    #[test]
    fn test_translate() {
        todo!()
    }

    #[test]
    fn test_square_itr() {
        todo!()
    }
}
