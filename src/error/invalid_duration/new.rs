use crate::InvalidDurationError;

impl InvalidDurationError {
    /// Creates a new [`InvalidDurationError`]
    pub fn new(duration: f64) -> Self {
        InvalidDurationError { duration }
    }
}
