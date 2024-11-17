use crate::Argument;
use std::env::Args;

mod new;
mod source;

/// Arguments sourced from the provided command-line options as strings
pub struct ArgsSource<'a> {
    /// The set of arguments
    args: Args,

    /// The first argument
    first: Argument<'a>,
}
