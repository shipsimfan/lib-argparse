use crate::{ArgumentSource, Flag, FlagInfo, Result};
use std::mem::MaybeUninit;

impl<T: Flag, const N: usize> Flag for [T; N] {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let info = info.drop_default();

        let mut array = MaybeUninit::uninit().transpose();
        for i in 0..N {
            array[i] = MaybeUninit::new(T::parse(source, &info, long)?);
        }

        Ok(unsafe { MaybeUninit::array_assume_init(array) })
    }
}
