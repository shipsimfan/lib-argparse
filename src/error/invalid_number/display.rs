use crate::InvalidNumberError;

impl std::fmt::Display for InvalidNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidNumberError::Invalid => "invalid number provided".fmt(f),
            InvalidNumberError::PosOverflow => "provided number is too large".fmt(f),
            InvalidNumberError::NegOverflow => "provided number is too small".fmt(f),
            InvalidNumberError::Zero => "value cannot be zero".fmt(f),
        }
    }
}
