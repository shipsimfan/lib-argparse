use crate::InvalidLengthError;

impl std::fmt::Display for InvalidLengthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidLengthError::TooShort => "value is too short",
            InvalidLengthError::TooLong => "value is too long",
        }
        .fmt(f)
    }
}
