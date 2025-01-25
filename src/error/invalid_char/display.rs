use crate::InvalidCharError;

impl std::fmt::Display for InvalidCharError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "expected exactly 1 character".fmt(f)
    }
}
