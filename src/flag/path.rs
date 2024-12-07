use crate::{Argument, ArgumentSource, Error, Flag, FlagInfo, Result};
use std::path::PathBuf;

impl Flag for PathBuf {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        Ok(
            match source.next().ok_or(Error::missing_flag_value(info, long))? {
                Argument::Str(str) => PathBuf::from(str.into_owned()),
                Argument::OsStr(os_str) => PathBuf::from(os_str.into_owned()),
            },
        )
    }
}
