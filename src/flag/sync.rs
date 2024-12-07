use crate::{ArgumentSource, Flag, FlagInfo, Result};
use std::sync::{Mutex, RwLock};

impl<T: Flag> Flag for Mutex<T> {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        T::parse(source, &info.drop_default(), long).map(Mutex::new)
    }
}

impl<T: Flag> Flag for RwLock<T> {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        T::parse(source, &info.drop_default(), long).map(RwLock::new)
    }
}
