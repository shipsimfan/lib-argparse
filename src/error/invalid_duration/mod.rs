mod display;
mod new;

/// A provided duration was invalid
#[derive(Debug)]
pub struct InvalidDurationError {
    /// The provided duration
    duration: f64,
}

impl std::error::Error for InvalidDurationError {}
