use crate::{ArgumentSource, Flag, FlagInfo, Result};

impl<T: Flag> Flag for Box<T> {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        T::parse(source, &info.drop_default(), long).map(Box::new)
    }
}
