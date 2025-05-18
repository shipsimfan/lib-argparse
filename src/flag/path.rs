use crate::{Argument, ArgumentSource, Error, Flag, FlagInfo, InvalidLengthError, Result};
use std::path::PathBuf;

impl Flag for PathBuf {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let path = match source.next().ok_or(Error::missing_flag_value(info, long))? {
            Argument::Str(str) => PathBuf::from(str.into_owned()),
            Argument::OsStr(os_str) => PathBuf::from(os_str.into_owned()),
        };

        if let Some(min) = info.min {
            if path.as_os_str().len() < min as _ {
                return Err(Error::invalid_flag_value(
                    info,
                    long,
                    InvalidLengthError::TooShort,
                ));
            }
        }

        if let Some(max) = info.max {
            if path.as_os_str().len() > max as _ {
                return Err(Error::invalid_flag_value(
                    info,
                    long,
                    InvalidLengthError::TooLong,
                ));
            }
        }

        Ok(path)
    }
}
