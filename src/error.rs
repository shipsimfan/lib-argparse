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

    /// An argument has invalid UTF-8
    InvalidUTF8,

    /// The argument stream is missing the zero argument
    MissingZeroArgument,

    /// An unexpected argument was encountered in the stream
    UnexpectedArgument,

    /// An unknown flag was passed
    UnknownFlag,

    /// A flag was repeated that cannot be
    RepeatedFlag,

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

    /// Creates a new [`Error`] with [`ErrorKind::InvalidUTF8`]
    ///
    /// ## Return Value
    /// Returns the newly created [`Error`]
    pub fn invalid_utf8() -> Self {
        Error::new(ErrorKind::InvalidUTF8, "invalid UTF-8 in argument")
    }

    /// Creates a new [`Error`] with [`ErrorKind::MissingZeroArgument`]
    ///
    /// ## Return Value
    /// Returns the newly created [`Error`]
    pub fn missing_zero_argument() -> Self {
        Error::new(
            ErrorKind::MissingZeroArgument,
            "missing the zero argument in the stream",
        )
    }

    /// Creates a new [`Error`] with [`ErrorKind::UnexpectedArgument`]
    ///
    /// ## Parameters
    ///  * `message` - The message to be displayed for the new [`Error`]
    ///
    /// ## Return Value
    /// Returns the newly created [`Error`]
    pub fn unexpected_argument<S: Into<Cow<'static, str>>>(message: S) -> Self {
        Error::new(ErrorKind::UnexpectedArgument, message)
    }

    /// Creates a new [`Error`] with [`ErrorKind::UnknownFlag`]
    ///
    /// ## Parameters
    ///  * `message` - The message to be displayed for the new [`Error`]
    ///
    /// ## Return Value
    /// Returns the newly created [`Error`]
    pub fn unknown_flag<S: Into<Cow<'static, str>>>(message: S) -> Self {
        Error::new(ErrorKind::UnknownFlag, message)
    }

    /// Creates a new [`Error`] with [`ErrorKind::RepeatedFlag`]
    ///
    /// ## Parameters
    ///  * `message` - The message to be displayed for the new [`Error`]
    ///
    /// ## Return Value
    /// Returns the newly created [`Error`]
    pub fn repeated_flag<S: Into<Cow<'static, str>>>(message: S) -> Self {
        Error::new(ErrorKind::RepeatedFlag, message)
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

impl From<std::ffi::OsString> for Error {
    fn from(_: std::ffi::OsString) -> Self {
        Error::invalid_utf8()
    }
}
