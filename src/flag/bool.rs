use crate::{ArgumentSource, Flag, FlagInfo, Result};
use std::sync::atomic::AtomicBool;

impl Flag for bool {
    fn parse(_: &mut dyn ArgumentSource, _: &FlagInfo<Self>, _: bool) -> Result<Self> {
        Ok(true)
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
    fn parse(_: &mut dyn ArgumentSource, _: &FlagInfo<Self>, _: bool) -> Result<Self> {
        Ok(AtomicBool::new(true))
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
