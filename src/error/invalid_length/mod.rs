mod display;

/// An invalid length value was passed
#[derive(Debug)]
pub enum InvalidLengthError {
    /// The value is too short
    TooShort,

    /// The value is too long
    TooLong,
}

impl std::error::Error for InvalidLengthError {}
