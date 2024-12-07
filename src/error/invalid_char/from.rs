use crate::InvalidCharError;

impl From<std::char::ParseCharError> for InvalidCharError {
    fn from(_: std::char::ParseCharError) -> Self {
        InvalidCharError
    }
}
