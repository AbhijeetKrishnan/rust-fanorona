use std::str;

use regex::Regex;

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

impl Direction {
    fn parse_dir(dir_str: &str) -> Option<Direction> {
        match dir_str {
            "N" | "n" => Some(Direction::North),
            "S" | "s" => Some(Direction::South),
            "E" | "e" => Some(Direction::East),
            "W" | "w" => Some(Direction::West),
            "NW" | "nw" | "nW" | "Nw" => Some(Direction::NorthWest),
            "NE" | "ne" | "nE" | "Ne" => Some(Direction::NorthEast),
            "SW" | "sw" | "sW" | "Sw" => Some(Direction::SouthWest),
            "SE" | "se" | "sE" | "Se" => Some(Direction::SouthEast),
            _ => None,
        }
    }
}

pub enum CaptureType {
    Approach,
    Withdrawal,
}

impl CaptureType {
    fn parse_capture(capture_str: &str) -> Option<CaptureType> {
        match capture_str {
            "F" | "f" => Some(CaptureType::Approach), // [F]orward
            "B" | "b" => Some(CaptureType::Withdrawal), // [B]ackward
            _ => None,
        }
    }
}

pub enum FanoronaMove {
    Move {
        from: (u8, u8),
        direction: Direction,
        capture_type: Option<CaptureType>,
    },
    EndTurn,
}

impl FanoronaMove {
    fn from_str_to_tuple(from_str: &str) -> Option<(u8, u8)> {
        let row = from_str.chars().nth(1).unwrap().to_digit(10)? as u8;
        let col = match from_str.chars().nth(0).unwrap() {
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
        Some((row, col))
    }

    pub fn parse_move_str(move_str: &'static str) -> Option<FanoronaMove> {
        let re = Regex::new(r"(?x)
            ^(?P<from>[a-iA-I][1-5])
            (?P<direction>n|s|e|w|nw|ne|sw|se|N|S|E|W|NW|NE|SW|SE)
            (?P<capture_type>[fbFB])?
            |
            ^(?P<end_turn>[Xx])
        ").ok()?;
        let caps = re.captures(move_str)?;
        match caps.name("end_turn") {
            Some(end_turn) => Some(FanoronaMove::EndTurn),
            None => {
                let from_str = caps.name("from")?.as_str();
                let from = Self::from_str_to_tuple(from_str)?;
                let dir_str = caps.name("direction")?.as_str();
                let direction = Direction::parse_dir(dir_str)?;
                let capture_type = caps.name("capture_type").map(|s| CaptureType::parse_capture(s.as_str()))?;
                Some(FanoronaMove::Move {
                    from,
                    direction,
                    capture_type,
                })
            },
        }
    }
}