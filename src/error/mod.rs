mod display;
mod error;
mod new;

/// A result of an argparse operation
pub type Result<T> = core::result::Result<T, Error>;

/// An error that can occur while parsing arguments
#[derive(Debug)]
pub enum Error {
    /// Invalid string with invalid UTF-8 was passed
    InvalidUTF8(String),

    /// A required argument is missing
    MissingArgument(&'static str),

    /// A required value for a positional is missing
    MissingPositionalValue(&'static str),

    /// An invalid value was given for a positional
    InvalidPositionalValue(&'static str, Box<dyn std::error::Error>),

    /// A required value for a flag is missing
    MissingFlagValue(&'static str, &'static str),

    /// An invalid value was given for a flag
    InvalidFlagValue(&'static str, &'static str, Box<dyn std::error::Error>),

    /// An unknown argument was passed
    UnknownArgument(String),

    /// A custom error used by consumers of this crate
    Custom(Box<dyn std::error::Error>),
}
