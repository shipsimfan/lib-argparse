use crate::UnexpectedError;

impl UnexpectedError {
    /// Creates a new [`UnexpectedError`]
    pub fn new<S: Into<String>>(unexpected: S, expected: &'static str) -> Self {
        UnexpectedError {
            unexpected: unexpected.into(),
            expected,
        }
    }
}
