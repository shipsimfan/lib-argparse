use crate::{ArgumentSource, Error, Flag, FlagInfo, Result};

impl<T: Flag> Flag for Option<T> {
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
        *this = Some(Some(T::unwrap(new, &info)?));
        Ok(())
    }

    fn unwrap(this: Option<Self>, _: &FlagInfo<Self>) -> Result<Self> {
        Ok(match this {
            Some(this) => this,
            None => None,
        })
    }

    fn is_required(_: &FlagInfo<Self>) -> bool {
        false
    }

    fn takes_value(info: &FlagInfo<Self>) -> bool {
        T::takes_value(&info.drop_default())
    }
}
