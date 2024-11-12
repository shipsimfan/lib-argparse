use crate::{messages::errors::INVALID_UTF8, Error};
use i18n::translation::m;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidUTF8(string) => {
                let string = string.to_string_lossy();
                m!(INVALID_UTF8, string => &string).fmt(f)
            }
        }
    }
}
