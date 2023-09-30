use std::{borrow::Cow, ffi::OsString};

/// Argument parsing errors
pub enum Error<E: 'static> {
    InvalidUTF8(OsString),
    MissingParameter(Cow<'static, str>),
    UnknowArgumentOS(OsString),
    UnknowArgument(String),
    RepeatedArgument(Cow<'static, str>),
    MissingArgument(Cow<'static, str>),
    UnexpectedArgument(OsString),
    UnknownCommandOS(OsString, Cow<'static, str>),
    UnknownCommand(String, Cow<'static, str>),
    MissingCommand(Cow<'static, str>),
    Other(E),
}

impl<E> Error<E> {
    pub fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidUTF8(string) => write!(f, "invalid utf-8 {:?}", string),
            Error::UnknowArgumentOS(argument) => write!(f, "unknown argument {:?}", argument),
            Error::UnknowArgument(argument) => write!(f, "unknown argument \"{}\"", argument),
            Error::UnexpectedArgument(argument) => write!(f, "unexpected argument {:?}", argument),
            Error::UnknownCommandOS(command, command_name) => {
                write!(f, "unknown {} {:?}", command_name, command)
            }
            Error::UnknownCommand(command, command_name) => {
                write!(f, "unknown {} \"{}\"", command_name, command)
            }

            Error::MissingParameter(message)
            | Error::RepeatedArgument(message)
            | Error::MissingArgument(message)
            | Error::MissingCommand(message) => write!(f, "{}", message),

            Error::Other(_) => write!(f, "other"),
        }
    }
}

impl<E: std::error::Error> std::error::Error for Error<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Other(error) => Some(error),

            Error::InvalidUTF8(_)
            | Error::MissingParameter(_)
            | Error::UnknowArgumentOS(_)
            | Error::UnknowArgument(_)
            | Error::RepeatedArgument(_)
            | Error::MissingArgument(_)
            | Error::UnexpectedArgument(_)
            | Error::UnknownCommandOS(_, _)
            | Error::UnknownCommand(_, _)
            | Error::MissingCommand(_) => None,
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
