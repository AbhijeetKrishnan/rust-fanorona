use std::fmt;

pub struct Square(u8);

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (row, col) = self.as_tuple();
        let row_str = row.to_string();
        let col_str = match col {
            0u8 => Some('A'),
            1u8 => Some('B'),
            2u8 => Some('C'),
            3u8 => Some('D'),
            4u8 => Some('E'),
            5u8 => Some('F'),
            6u8 => Some('G'),
            7u8 => Some('H'),
            8u8 => Some('I'),
            _ => fmt::Error,
        }?;
        write!(f, "{}{}", row_str, col_str)
    }
}

impl From<(u8, u8)> for Square {
    fn from(move_tuple: (u8, u8)) -> Square {
        Square(move_tuple.0 * 9 + move_tuple.1)
    }
}

impl TryFrom<&str> for Square {
    fn try_from(square_str: &str) -> Result<Square, Square::Error> {
        let row = square_str.chars().nth(1)?.to_digit(10)? as u8;
        let col = match square_str.chars().nth(0)? {
            'a' | 'A' => Some(0u8),
            'b' | 'B' => Some(1u8),
            'c' | 'C' => Some(2u8),
            'd' | 'D' => Some(3u8),
            'e' | 'E' => Some(4u8),
            'f' | 'F' => Some(5u8),
            'g' | 'G' => Some(6u8),
            'h' | 'H' => Some(7u8),
            'i' | 'I' => Some(8u8),
            _ => None,
        }?;
        Some(Square::from((row, col)))
    }
}

impl Square {
    pub fn as_tuple(&self) -> (u8, u8) {
        (self.0 / 9, self.0 % 9)
    }

    pub fn from_str(square_str: &str) -> Option<Square> {
        let row = square_str.chars().nth(1)?.to_digit(10)? as u8;
        let col = match square_str.chars().nth(0)? {
            'a' | 'A' => Some(0u8),
            'b' | 'B' => Some(1u8),
            'c' | 'C' => Some(2u8),
            'd' | 'D' => Some(3u8),
            'e' | 'E' => Some(4u8),
            'f' | 'F' => Some(5u8),
            'g' | 'G' => Some(6u8),
            'h' | 'H' => Some(7u8),
            'i' | 'I' => Some(8u8),
            _ => None,
        }?;
        Some(Square::from_tuple())
    }
}