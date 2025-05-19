use crate::{Argument, Error, InvalidLengthError, Positional, PositionalInfo, PositionalResult};
use std::path::PathBuf;

impl Positional for PathBuf {
    fn parse<'a>(
        this: &mut Option<Self>,
        argument: Argument<'a>,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult<'a> {
        let path = match argument {
            Argument::Str(str) => PathBuf::from(str.into_owned()),
            Argument::OsStr(os_str) => PathBuf::from(os_str.into_owned()),
        };

        if let Some(min) = info.min {
            if path.as_os_str().len() < min as _ {
                return PositionalResult::Error(Error::invalid_positional_value(
                    info.value,
                    InvalidLengthError::TooShort,
                ));
            }
        }

        if let Some(max) = info.max {
            if path.as_os_str().len() > max as _ {
                return PositionalResult::Error(Error::invalid_positional_value(
                    info.value,
                    InvalidLengthError::TooLong,
                ));
            }
        }

        *this = Some(path);
        PositionalResult::Next
    }
}
