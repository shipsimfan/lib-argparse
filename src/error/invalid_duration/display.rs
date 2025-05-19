use crate::InvalidDurationError;

impl std::fmt::Display for InvalidDurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid duration \"{}\"", self.duration)
    }
}
