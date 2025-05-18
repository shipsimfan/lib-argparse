use crate::{ArgumentSource, Error, Flag, FlagInfo, Result};
use std::sync::{Mutex, RwLock};

impl<T: Flag> Flag for Mutex<T> {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        if this.is_some() {
            return Err(Error::repeated_flag(info, long));
        }

        let mut new = None;
        let info = info.drop_default();
        T::parse(&mut new, source, &info, long)?;
        *this = Some(Mutex::new(T::unwrap(new, &info)?));
        Ok(())
    }

    fn takes_value(info: &FlagInfo<Self>) -> bool {
        T::takes_value(&info.drop_default())
    }
}

impl<T: Flag> Flag for RwLock<T> {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        if this.is_some() {
            return Err(Error::repeated_flag(info, long));
        }

        let mut new = None;
        let info = info.drop_default();
        T::parse(&mut new, source, &info, long)?;
        *this = Some(RwLock::new(T::unwrap(new, &info)?));
        Ok(())
    }

    fn takes_value(info: &FlagInfo<Self>) -> bool {
        T::takes_value(&info.drop_default())
    }
}
