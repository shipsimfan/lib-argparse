use crate::{ArgumentSource, Flag, FlagInfo, Result};
use std::cell::RefCell;

impl<T: Flag> Flag for RefCell<T> {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        T::parse(source, &info.drop_default(), long).map(RefCell::new)
    }

    fn takes_value(info: &FlagInfo<Self>) -> bool {
        T::takes_value(&info.drop_default())
    }
}
