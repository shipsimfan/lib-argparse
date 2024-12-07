use crate::{ArgumentSource, Error, Flag, FlagInfo, InvalidCharError, Result};

impl Flag for char {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidCharError::from(error)))
    }
}
