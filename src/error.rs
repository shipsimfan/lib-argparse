use std::borrow::Cow;

/// A result of parsing arguments
pub type Result<T> = std::result::Result<T, Error>;

/// A class of error that occurred
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// Not enough parameters passed to the argument
    MissingParameters,

    /// A required argument wasn't provided
    MissingRequired,

    /// Another error
    Custom,
}

/// An error that occurred while parsing arguments
pub struct Error {
    kind: ErrorKind,
    message: Cow<'static, str>,
}

impl Error {
    /// Creates a new [`Error`]
    ///
    /// ## Parameters
    ///  * `kind` - The [`ErrorKind`] for the new [`Error`]
    ///  * `message` - The message to be displayed for the new [`Error`]
    ///
    /// ## Return Value
    /// Returns the newly created [`Error`]
    pub fn new<S: Into<Cow<'static, str>>>(kind: ErrorKind, message: S) -> Self {
        Error {
            kind,
            message: message.into(),
        }
    }

    /// Creates a new [`Error`] with [`ErrorKind::MissingParameters`]
    ///
    /// ## Parameters
    ///  * `message` - The message to be displayed for the new [`Error`]
    ///
    /// ## Return Value
    /// Returns the newly created [`Error`]
    pub fn missing_parameters<S: Into<Cow<'static, str>>>(message: S) -> Self {
        Error::new(ErrorKind::MissingParameters, message)
    }

    /// Creates a new [`Error`] with [`ErrorKind::MissingRequired`]
    ///
    /// ## Parameters
    ///  * `message` - The message to be displayed for the new [`Error`]
    ///
    /// ## Return Value
    /// Returns the newly created [`Error`]
    pub fn missing_required<S: Into<Cow<'static, str>>>(message: S) -> Self {
        Error::new(ErrorKind::MissingRequired, message)
    }

    /// Creates a new [`Error`] with [`ErrorKind::Custom`]
    ///
    /// ## Parameters
    ///  * `message` - The message to be displayed for the new [`Error`]
    ///
    /// ## Return Value
    /// Returns the newly created [`Error`]
    pub fn custom<S: Into<Cow<'static, str>>>(message: S) -> Self {
        Error::new(ErrorKind::Custom, message)
    }

    /// Gets the kind of error this is
    ///
    /// ## Return Value
    /// Returns the [`ErrorKind`] of this error
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Gets the message for this error
    ///
    /// ## Return Value
    /// Returns the message for the error
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
