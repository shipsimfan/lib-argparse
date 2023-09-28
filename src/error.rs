use crate::InvalidUTF8;
use std::borrow::Cow;

/// Argument parsing errors
pub enum Error<E: 'static> {
    InvalidUTF8,
    MissingParameter(Cow<'static, str>),
    Other(E),
}

impl<E> Error<E> {
    pub fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidUTF8 => write!(f, "invalid utf-8"),
            Error::MissingParameter(message) => write!(f, "{}", message),

            Error::Other(_) => write!(f, "other"),
        }
    }
}

impl<E: std::error::Error> std::error::Error for Error<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Other(error) => Some(error),

            Error::InvalidUTF8 | Error::MissingParameter(_) => None,
        }
    }
}

impl<E: std::fmt::Display> std::fmt::Display for Error<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Other(error) => error.fmt(f),

            _ => Self::fmt(self, f),
        }
    }
}

impl<E: std::fmt::Debug> std::fmt::Debug for Error<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Other(error) => error.fmt(f),

            _ => Self::fmt(self, f),
        }
    }
}

impl<E> From<InvalidUTF8> for Error<E> {
    fn from(_: InvalidUTF8) -> Self {
        Error::InvalidUTF8
    }
}
