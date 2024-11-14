use crate::Error;

impl Error {
    /// Create an [`Error::Custom`] containing `message`
    pub fn custom<T: std::fmt::Display>(message: T) -> Self {
        Error::Custom(message.to_string())
    }

    /// Create an [`Error::InvalidUTF8`] for `string`
    pub fn invalid_utf8<T: Into<String>>(string: T) -> Self {
        Error::InvalidUTF8(string.into())
    }
}
