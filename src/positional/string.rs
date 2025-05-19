use crate::{Argument, Error, InvalidLengthError, Positional, PositionalInfo, PositionalResult};
use std::ffi::OsString;

impl Positional for String {
    fn parse<'a>(
        this: &mut Option<Self>,
        argument: Argument<'a>,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult<'a> {
        let str = match argument {
            Argument::Str(str) => str.into_owned(),
            Argument::OsStr(os_str) => match os_str.as_str() {
                Ok(string) => string.to_owned(),
                Err(error) => return PositionalResult::Error(error),
            },
        };

        if let Some(min) = info.min {
            if str.len() < min as _ {
                return PositionalResult::Error(Error::invalid_positional_value(
                    info.value,
                    InvalidLengthError::TooShort,
                ));
            }
        }

        if let Some(max) = info.max {
            if str.len() > max as _ {
                return PositionalResult::Error(Error::invalid_positional_value(
                    info.value,
                    InvalidLengthError::TooLong,
                ));
            }
        }

        *this = Some(str);
        PositionalResult::Next
    }
}

impl Positional for OsString {
    fn parse<'a>(
        this: &mut Option<Self>,
        argument: Argument<'a>,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult<'a> {
        let str = match argument {
            Argument::Str(str) => OsString::from(str.as_ref()),
            Argument::OsStr(os_str) => os_str.to_os_string(),
        };

        if let Some(min) = info.min {
            if str.len() < min as _ {
                return PositionalResult::Error(Error::invalid_positional_value(
                    info.value,
                    InvalidLengthError::TooShort,
                ));
            }
        }

        if let Some(max) = info.max {
            if str.len() > max as _ {
                return PositionalResult::Error(Error::invalid_positional_value(
                    info.value,
                    InvalidLengthError::TooLong,
                ));
            }
        }

        *this = Some(str);
        PositionalResult::Next
    }
}
