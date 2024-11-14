use crate::{messages::errors::*, Error};
use i18n::translation::m;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidUTF8(string) => m!(InvalidUTF8, string).fmt(f),
            Error::MissingArgument(argument) => m!(MissingArgument, argument).fmt(f),
            Error::MissingValue(argument, value) => m!(MissingValue, argument, value).fmt(f),
            Error::InvalidValue(argument, value, error) => {
                m!(InvalidValue, argument, value, error).fmt(f)
            }
            Error::UnknownArgument(argument) => m!(UnknownArgument, argument).fmt(f),
            Error::Custom(message) => message.fmt(f),
        }
    }
}
