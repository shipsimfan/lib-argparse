use std::env::Args;

mod new;
mod source;

/// Arguments sourced from the provided command-line options as strings
pub struct ArgsSource {
    /// The set of arguments
    args: Args,
}
