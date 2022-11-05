use std::{fmt, string::String};

#[derive(Debug)]
pub enum SquareError {
    TryFromStrError(String),
}

impl std::error::Error for SquareError {}

impl fmt::Display for SquareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SquareError::TryFromStrError(msg) => write!(f, "{}", msg),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Square(u8);

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (row, col): (u8, u8) = (*self).into();
        let row_str = row.to_string();
        let col_str = match col {
            0u8 => Ok('A'),
            1u8 => Ok('B'),
            2u8 => Ok('C'),
            3u8 => Ok('D'),
            4u8 => Ok('E'),
            5u8 => Ok('F'),
            6u8 => Ok('G'),
            7u8 => Ok('H'),
            8u8 => Ok('I'),
            _ => Err(fmt::Error),
        }?;
        write!(f, "{}{}", row_str, col_str)
    }
}

impl From<(u8, u8)> for Square {
    fn from(move_tuple: (u8, u8)) -> Square {
        Square(move_tuple.0 * 9 + move_tuple.1)
    }
}

impl Into<(u8, u8)> for Square {
    fn into(self) -> (u8, u8) {
        (self.0 / 9, self.0 % 9)
    }
}

impl TryFrom<&str> for Square {
    type Error = SquareError;
    fn try_from(square_str: &str) -> Result<Square, SquareError> {
        let row = square_str
            .chars()
            .nth(1)
            .ok_or_else(|| SquareError::TryFromStrError(String::from("row char does not exist")))?
            .to_digit(10)
            .ok_or_else(|| {
                SquareError::TryFromStrError(String::from("could not convert row to number"))
            })? as u8;
        let col =
            match square_str.chars().nth(0).ok_or_else(|| {
                SquareError::TryFromStrError(String::from("col char does not exist"))
            })? {
                'a' | 'A' => Ok(0u8),
                'b' | 'B' => Ok(1u8),
                'c' | 'C' => Ok(2u8),
                'd' | 'D' => Ok(3u8),
                'e' | 'E' => Ok(4u8),
                'f' | 'F' => Ok(5u8),
                'g' | 'G' => Ok(6u8),
                'h' | 'H' => Ok(7u8),
                'i' | 'I' => Ok(8u8),
                _ => Err(SquareError::TryFromStrError(String::from(
                    "could not convert col to number",
                ))),
            }?;
        Ok(Square::from((row, col)))
    }
}

impl Square {
    
    #[inline]
    pub fn idx(&self) -> usize {
        self.0 as usize
    }
}