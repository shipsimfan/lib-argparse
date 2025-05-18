use crate::{ArgumentSource, Error, Flag, FlagInfo, Result};
use std::sync::atomic::AtomicBool;

impl Flag for bool {
    fn parse(
        this: &mut Option<Self>,
        _: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        if this.is_some() {
            return Err(Error::repeated_flag(info, long));
        }

        *this = Some(true);
        Ok(())
    }

    fn unwrap(this: Option<Self>, _: &FlagInfo<Self>) -> Result<Self> {
        Ok(this.unwrap_or(false))
    }

    fn is_required(_: &FlagInfo<Self>) -> bool {
        false
    }

    fn takes_value(_: &FlagInfo<Self>) -> bool {
        false
    }
}

impl Flag for AtomicBool {
    fn parse(
        this: &mut Option<Self>,
        _: &mut dyn ArgumentSource,
        _: &FlagInfo<Self>,
        _: bool,
    ) -> Result<()> {
        *this = Some(AtomicBool::new(true));
        Ok(())
    }

    fn unwrap(this: Option<Self>, _: &FlagInfo<Self>) -> Result<Self> {
        Ok(this.unwrap_or(AtomicBool::new(false)))
    }

    fn is_required(_: &FlagInfo<Self>) -> bool {
        false
    }

    fn takes_value(_: &FlagInfo<Self>) -> bool {
        false
    }
}
