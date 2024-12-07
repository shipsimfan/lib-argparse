use crate::{ArgumentSource, Error, Flag, FlagInfo, Result};

impl Flag for String {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        Ok(source
            .next()
            .ok_or(Error::missing_flag_value(info, long))?
            .as_str()?
            .to_owned())
    }
}
