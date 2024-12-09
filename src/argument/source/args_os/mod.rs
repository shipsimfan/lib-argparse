use std::env::ArgsOs;

mod new;
mod source;

/// Arguments sourced from the provided command-line options as os strings
pub struct ArgsOsSource {
    /// The set of arguments
    args: ArgsOs,
}
