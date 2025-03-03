use crate::FanoronaError;
use std::fmt;

/// A representation of the capture type of a move
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CaptureType {
    Approach,
    Withdrawal,
}

impl fmt::Display for CaptureType {
    /// Print the capture type as [F]orward, or [B]ackward
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let capture_type_str = match self {
            Self::Approach => 'F',
            Self::Withdrawal => 'B',
        };
        write!(f, "{}", capture_type_str)
    }
}

impl TryFrom<&str> for CaptureType {
    type Error = FanoronaError;

    /// Parse a capture type string (case-insensitive) into a CaptureType
    fn try_from(capture_type_str: &str) -> Result<CaptureType, FanoronaError> {
        match capture_type_str {
            "F" | "f" => Ok(CaptureType::Approach),   // [F]orward
            "B" | "b" => Ok(CaptureType::Withdrawal), // [B]ackward
            _ => Err(FanoronaError::TryFromStrError(String::from(format!(
                "could not parse {} as capture type",
                capture_type_str
            )))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!("F", CaptureType::Approach.to_string());
        assert_eq!("B", CaptureType::Withdrawal.to_string());
    }

    #[test]
    fn test_try_from() {
        assert_eq!(CaptureType::Approach, CaptureType::try_from("f").unwrap());
        assert_eq!(CaptureType::Approach, CaptureType::try_from("F").unwrap());

        assert_eq!(CaptureType::Withdrawal, CaptureType::try_from("b").unwrap());
        assert_eq!(CaptureType::Withdrawal, CaptureType::try_from("B").unwrap());

        assert!(CaptureType::try_from("x").is_err());
        assert!(CaptureType::try_from("xyz").is_err());
        assert!(CaptureType::try_from("1").is_err());
        assert!(CaptureType::try_from("fb").is_err());
        assert!(CaptureType::try_from("BF").is_err());
    }
}
