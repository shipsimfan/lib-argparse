mod display;
mod from;

/// An invalid character was parsed
#[derive(Debug)]
pub struct InvalidCharError;

impl std::error::Error for InvalidCharError {}
