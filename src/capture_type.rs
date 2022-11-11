use std::fmt;

#[derive(Debug)]
pub enum CaptureTypeError {
    TryFromStrError(String),
}

impl std::error::Error for CaptureTypeError {}

impl fmt::Display for CaptureTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CaptureTypeError::TryFromStrError(msg) => write!(f, "{}", msg),
        }
    }
}

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
    type Error = CaptureTypeError;
    fn try_from(capture_type_str: &str) -> Result<CaptureType, CaptureTypeError> {
        match capture_type_str {
            "F" | "f" => Ok(CaptureType::Approach),   // [F]orward
            "B" | "b" => Ok(CaptureType::Withdrawal), // [B]ackward
            _ => Err(CaptureTypeError::TryFromStrError(String::from(format!(
                "could not parse {} as capture type",
                capture_type_str
            )))),
        }
    }
}
