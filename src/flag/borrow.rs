use crate::{ArgumentSource, DefaultDisplay, Flag, FlagInfo, Result};
use std::borrow::Cow;

impl<'a, T: Flag, B: DefaultDisplay + ToOwned<Owned = T>> Flag for Cow<'a, B> {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        T::parse(source, &info.drop_default(), long).map(Cow::Owned)
    }
}
