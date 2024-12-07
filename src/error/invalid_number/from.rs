use crate::InvalidNumberError;

impl From<std::num::ParseIntError> for InvalidNumberError {
    fn from(value: std::num::ParseIntError) -> Self {
        match *value.kind() {
            std::num::IntErrorKind::PosOverflow => InvalidNumberError::PosOverflow,
            std::num::IntErrorKind::NegOverflow => InvalidNumberError::NegOverflow,
            std::num::IntErrorKind::Zero => InvalidNumberError::Zero,
            _ => InvalidNumberError::Invalid,
        }
    }
}

impl From<std::num::ParseFloatError> for InvalidNumberError {
    fn from(_: std::num::ParseFloatError) -> Self {
        InvalidNumberError::Invalid
    }
}
