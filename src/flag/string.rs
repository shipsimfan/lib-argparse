use crate::{Argument, ArgumentSource, Error, Flag, FlagInfo, Result};
use std::ffi::OsString;

impl Flag for String {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        Ok(source
            .next()
            .ok_or(Error::missing_flag_value(info, long))?
            .as_str()?
            .to_owned())
    }
}

impl Flag for OsString {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        Ok(
            match source.next().ok_or(Error::missing_flag_value(info, long))? {
                Argument::OsStr(os_str) => os_str.into_owned(),
                Argument::Str(str) => OsString::from(str.as_ref()),
            },
        )
    }
}
