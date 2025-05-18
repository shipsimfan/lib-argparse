use crate::{Argument, ArgumentSource, Error, Flag, FlagInfo, InvalidLengthError, Result};
use std::ffi::OsString;

impl Flag for String {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let str = source
            .next()
            .ok_or(Error::missing_flag_value(info, long))?
            .as_str()?
            .to_owned();

        if let Some(min) = info.min {
            if str.len() < min as _ {
                return Err(Error::invalid_flag_value(
                    info,
                    long,
                    InvalidLengthError::TooShort,
                ));
            }
        }

        if let Some(max) = info.max {
            if str.len() > max as _ {
                return Err(Error::invalid_flag_value(
                    info,
                    long,
                    InvalidLengthError::TooLong,
                ));
            }
        }

        Ok(str)
    }
}

impl Flag for OsString {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let str = match source.next().ok_or(Error::missing_flag_value(info, long))? {
            Argument::OsStr(os_str) => os_str.into_owned(),
            Argument::Str(str) => OsString::from(str.as_ref()),
        };

        if let Some(min) = info.min {
            if str.len() < min as _ {
                return Err(Error::invalid_flag_value(
                    info,
                    long,
                    InvalidLengthError::TooShort,
                ));
            }
        }

        if let Some(max) = info.max {
            if str.len() > max as _ {
                return Err(Error::invalid_flag_value(
                    info,
                    long,
                    InvalidLengthError::TooLong,
                ));
            }
        }

        Ok(str)
    }
}
