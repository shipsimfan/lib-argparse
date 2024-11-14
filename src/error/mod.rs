mod display;
mod error;
mod new;

/// An error that can occur while parsing arguments
#[derive(Debug)]
pub enum Error {
    /// Invalid string with invalid UTF-8 was passed
    InvalidUTF8(String),

    /// A custom error used by consumers of this crate
    Custom(String),
}
