use crate::{ArgumentSource, Error, Flag, FlagInfo, InvalidCharError, Result};

impl Flag for char {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        if this.is_some() {
            return Err(Error::repeated_flag(info, long));
        }

        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        *this = Some(value.as_str()?.parse().map_err(|error| {
            Error::invalid_flag_value(info, long, InvalidCharError::from(error))
        })?);
        Ok(())
    }
}
