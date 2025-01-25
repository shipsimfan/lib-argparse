use crate::InvalidAddressError;

impl std::fmt::Display for InvalidAddressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "invalid address provided".fmt(f)
    }
}
