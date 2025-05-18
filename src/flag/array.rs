use crate::{ArgumentSource, Error, Flag, FlagInfo, Result};
use std::mem::MaybeUninit;

impl<T: Flag, const N: usize> Flag for [T; N] {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        if this.is_some() {
            return Err(Error::repeated_flag(info, long));
        }

        let info = info.drop_default();

        let mut array = MaybeUninit::uninit().transpose();
        for i in 0..N {
            let mut new = None;
            T::parse(&mut new, source, &info, long)?;
            array[i] = MaybeUninit::new(T::unwrap(new, &info.drop_default())?);
        }

        *this = Some(unsafe { MaybeUninit::array_assume_init(array) });
        Ok(())
    }
}
