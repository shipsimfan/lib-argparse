use crate::{ArgumentSource, DefaultDisplay, Error, Flag, FlagInfo, Result};
use std::borrow::Cow;

impl<'a, T: Flag, B: DefaultDisplay + ToOwned<Owned = T> + ?Sized> Flag for Cow<'a, B> {
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
        *this = Some(Cow::Owned(T::unwrap(new, &info)?));
        Ok(())
    }

    fn takes_value(info: &FlagInfo<Self>) -> bool {
        T::takes_value(&info.drop_default())
    }
}
