use crate::{ArgumentSource, Flag, FlagInfo, Result};

impl Flag for bool {
    fn parse(_: &mut dyn ArgumentSource, _: &FlagInfo<Self>) -> Result<Self> {
        Ok(true)
    }

    fn unwrap(this: Option<Self>, _: &FlagInfo<Self>) -> Result<Self> {
        Ok(this.unwrap_or(false))
    }
}
