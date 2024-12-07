mod display;
mod from;

/// An invalid number was parsed
#[derive(Debug)]
pub enum InvalidNumberError {
    /// The value provided was not a valid number
    Invalid,

    /// The value provided is too large
    PosOverflow,

    /// The value provided is too small
    NegOverflow,

    /// The value provided is zero when it shouldn't be
    Zero,
}

impl std::error::Error for InvalidNumberError {}
