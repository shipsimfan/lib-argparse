use crate::{messages::errors::*, ArgumentSource, Error, Flag, FlagInfo, Result};
use i18n::translation::m;

/// An invalid character was parsed
#[derive(Debug)]
pub struct InvalidCharError;

impl std::error::Error for InvalidCharError {}

impl std::fmt::Display for InvalidCharError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        m!(CharInvalid).fmt(f)
    }
}

impl From<std::char::ParseCharError> for InvalidCharError {
    fn from(_: std::char::ParseCharError) -> Self {
        InvalidCharError
    }
}

impl Flag for char {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}
