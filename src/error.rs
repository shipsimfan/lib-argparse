/// A result of parsing arguments
pub type Result<'a, T> = std::result::Result<T, Error<'a>>;

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

pub enum ErrorMessage<'a> {
    Display(&'a dyn std::fmt::Display),
    Str(&'a str),
    Owned(String),
}

/// An error that occurred while parsing arguments
pub struct Error<'a> {
    kind: ErrorKind,
    message: ErrorMessage<'a>,
}

impl<'a> Error<'a> {
    /// Creates a new [`Error`]
    ///
    /// ## Parameters
    ///  * `kind` - The [`ErrorKind`] for the new [`Error`]
    ///  * `message` - The message to be displayed for the new [`Error`]
    ///
    /// ## Return Value
    /// Returns the newly created [`Error`]
    pub fn new<S: Into<ErrorMessage<'a>>>(kind: ErrorKind, message: S) -> Self {
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
    pub fn missing_parameters<S: Into<ErrorMessage<'a>>>(message: S) -> Self {
        Error::new(ErrorKind::MissingParameters, message)
    }

    /// Creates a new [`Error`] with [`ErrorKind::MissingRequired`]
    ///
    /// ## Parameters
    ///  * `message` - The message to be displayed for the new [`Error`]
    ///
    /// ## Return Value
    /// Returns the newly created [`Error`]
    pub fn missing_required<S: Into<ErrorMessage<'a>>>(message: S) -> Self {
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
    pub fn unexpected_argument<S: Into<ErrorMessage<'a>>>(message: S) -> Self {
        Error::new(ErrorKind::UnexpectedArgument, message)
    }

    /// Creates a new [`Error`] with [`ErrorKind::UnknownFlag`]
    ///
    /// ## Parameters
    ///  * `message` - The message to be displayed for the new [`Error`]
    ///
    /// ## Return Value
    /// Returns the newly created [`Error`]
    pub fn unknown_flag<S: Into<ErrorMessage<'a>>>(message: S) -> Self {
        Error::new(ErrorKind::UnknownFlag, message)
    }

    /// Creates a new [`Error`] with [`ErrorKind::RepeatedFlag`]
    ///
    /// ## Parameters
    ///  * `message` - The message to be displayed for the new [`Error`]
    ///
    /// ## Return Value
    /// Returns the newly created [`Error`]
    pub fn repeated_flag<S: Into<ErrorMessage<'a>>>(message: S) -> Self {
        Error::new(ErrorKind::RepeatedFlag, message)
    }

    /// Creates a new [`Error`] with [`ErrorKind::Custom`]
    ///
    /// ## Parameters
    ///  * `message` - The message to be displayed for the new [`Error`]
    ///
    /// ## Return Value
    /// Returns the newly created [`Error`]
    pub fn custom<S: Into<ErrorMessage<'a>>>(message: S) -> Self {
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
    pub fn message(&self) -> &dyn std::fmt::Display {
        &self.message
    }

    /// Converts the contents to be owned and no longer borrowed
    pub fn to_owned(self) -> Error<'static> {
        Error {
            kind: self.kind,
            message: self.message.to_owned(),
        }
    }
}

impl<'a> std::error::Error for Error<'a> {}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

impl<'a> std::fmt::Debug for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl<'a> From<std::ffi::OsString> for Error<'a> {
    fn from(_: std::ffi::OsString) -> Self {
        Error::invalid_utf8()
    }
}

impl<'a> ErrorMessage<'a> {
    pub fn to_owned(self) -> ErrorMessage<'static> {
        match self {
            ErrorMessage::Display(display) => ErrorMessage::Owned(display.to_string()),
            ErrorMessage::Str(str) => ErrorMessage::Owned(str.to_owned()),
            ErrorMessage::Owned(string) => ErrorMessage::Owned(string),
        }
    }
}

impl<'a> std::fmt::Display for ErrorMessage<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorMessage::Display(display) => display.fmt(f),
            ErrorMessage::Str(str) => f.write_str(str),
            ErrorMessage::Owned(owned) => f.write_str(owned),
        }
    }
}

impl<'a, T: std::fmt::Display> From<&'a T> for ErrorMessage<'a> {
    fn from(display: &'a T) -> Self {
        ErrorMessage::Display(display)
    }
}

impl<'a> From<&'a str> for ErrorMessage<'a> {
    fn from(value: &'a str) -> Self {
        ErrorMessage::Str(value)
    }
}

impl<'a> From<String> for ErrorMessage<'a> {
    fn from(string: String) -> Self {
        ErrorMessage::Owned(string)
    }
}
