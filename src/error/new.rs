use crate::Error;

impl Error {
    /// Create an [`Error::Custom`] containing `message`
    pub fn custom<T: 'static + std::error::Error>(message: T) -> Self {
        Error::Custom(Box::new(message))
    }

    /// Create an [`Error::InvalidUTF8`] for `string`
    pub fn invalid_utf8<T: Into<String>>(string: T) -> Self {
        Error::InvalidUTF8(string.into())
    }

    /// Create an [`Error::MissingArgument`] for `argument`
    pub fn missing_argument(argument: &'static str) -> Self {
        Error::MissingArgument(argument)
    }

    /// Create an [`Error::MissingValue`] for `value` of `argument`
    pub fn missing_value(argument: &'static str, value: &'static str) -> Self {
        Error::MissingValue(argument, value)
    }

    /// Create an [`Error::InvalidValue`] for `value` of `argument`
    pub fn invalid_value<T: 'static + std::error::Error>(
        argument: &'static str,
        value: &'static str,
        error: T,
    ) -> Self {
        Error::InvalidValue(argument, value, Box::new(error))
    }

    /// Create an [`Error::UnknownArgument`] for `argument`
    pub fn unknown_argument(argument: String) -> Self {
        Error::UnknownArgument(argument)
    }
}
