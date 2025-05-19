mod display;
mod new;

/// A provided value was unexpected
#[derive(Debug)]
pub struct UnexpectedError {
    /// The unexpected value
    unexpected: String,

    /// The values that were expected
    expected: &'static str,
}

impl std::error::Error for UnexpectedError {}
