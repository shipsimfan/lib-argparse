use crate::Argument;
use std::env::ArgsOs;

mod new;
mod source;

/// Arguments sourced from the provided command-line options as os strings
pub struct ArgsOsSource<'a> {
    /// The set of arguments
    args: ArgsOs,

    /// The first argument
    first: Argument<'a>,
}
