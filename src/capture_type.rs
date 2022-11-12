use crate::FanoronaError;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CaptureType {
    Approach,
    Withdrawal,
}

impl fmt::Display for CaptureType {
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
