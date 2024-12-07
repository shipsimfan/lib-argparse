mod display;

/// A provided address was incorrect
#[derive(Debug)]
pub struct InvalidAddressError;

impl std::error::Error for InvalidAddressError {}
