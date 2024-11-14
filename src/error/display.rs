use crate::{messages::errors::InvalidUTF8, Error};
use i18n::translation::m;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidUTF8(string) => m!(InvalidUTF8, string).fmt(f),
            Error::Custom(message) => message.fmt(f),
        }
    }
}
