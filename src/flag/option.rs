use crate::{ArgumentSource, Flag, FlagInfo, Result};

impl<T: Flag> Flag for Option<T> {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        T::parse(source, &info.drop_default(), long).map(Some)
    }

    fn unwrap(this: Option<Self>, _: &FlagInfo<Self>) -> Result<Self> {
        Ok(match this {
            Some(this) => this,
            None => None,
        })
    }
}
