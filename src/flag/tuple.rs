use crate::{ArgumentSource, Flag, FlagInfo, Result};

impl<T1: Flag, T2: Flag> Flag for (T1, T2) {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        Ok((
            T1::parse(source, &info.drop_default(), long)?,
            T2::parse(source, &info.drop_default(), long)?,
        ))
    }
}

impl<T1: Flag, T2: Flag, T3: Flag> Flag for (T1, T2, T3) {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        Ok((
            T1::parse(source, &info.drop_default(), long)?,
            T2::parse(source, &info.drop_default(), long)?,
            T3::parse(source, &info.drop_default(), long)?,
        ))
    }
}

impl<T1: Flag, T2: Flag, T3: Flag, T4: Flag> Flag for (T1, T2, T3, T4) {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        Ok((
            T1::parse(source, &info.drop_default(), long)?,
            T2::parse(source, &info.drop_default(), long)?,
            T3::parse(source, &info.drop_default(), long)?,
            T4::parse(source, &info.drop_default(), long)?,
        ))
    }
}

impl<T1: Flag, T2: Flag, T3: Flag, T4: Flag, T5: Flag> Flag for (T1, T2, T3, T4, T5) {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        Ok((
            T1::parse(source, &info.drop_default(), long)?,
            T2::parse(source, &info.drop_default(), long)?,
            T3::parse(source, &info.drop_default(), long)?,
            T4::parse(source, &info.drop_default(), long)?,
            T5::parse(source, &info.drop_default(), long)?,
        ))
    }
}

impl<T1: Flag, T2: Flag, T3: Flag, T4: Flag, T5: Flag, T6: Flag> Flag for (T1, T2, T3, T4, T5, T6) {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        Ok((
            T1::parse(source, &info.drop_default(), long)?,
            T2::parse(source, &info.drop_default(), long)?,
            T3::parse(source, &info.drop_default(), long)?,
            T4::parse(source, &info.drop_default(), long)?,
            T5::parse(source, &info.drop_default(), long)?,
            T6::parse(source, &info.drop_default(), long)?,
        ))
    }
}

impl<T1: Flag, T2: Flag, T3: Flag, T4: Flag, T5: Flag, T6: Flag, T7: Flag> Flag
    for (T1, T2, T3, T4, T5, T6, T7)
{
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        Ok((
            T1::parse(source, &info.drop_default(), long)?,
            T2::parse(source, &info.drop_default(), long)?,
            T3::parse(source, &info.drop_default(), long)?,
            T4::parse(source, &info.drop_default(), long)?,
            T5::parse(source, &info.drop_default(), long)?,
            T6::parse(source, &info.drop_default(), long)?,
            T7::parse(source, &info.drop_default(), long)?,
        ))
    }
}

impl<T1: Flag, T2: Flag, T3: Flag, T4: Flag, T5: Flag, T6: Flag, T7: Flag, T8: Flag> Flag
    for (T1, T2, T3, T4, T5, T6, T7, T8)
{
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        Ok((
            T1::parse(source, &info.drop_default(), long)?,
            T2::parse(source, &info.drop_default(), long)?,
            T3::parse(source, &info.drop_default(), long)?,
            T4::parse(source, &info.drop_default(), long)?,
            T5::parse(source, &info.drop_default(), long)?,
            T6::parse(source, &info.drop_default(), long)?,
            T7::parse(source, &info.drop_default(), long)?,
            T8::parse(source, &info.drop_default(), long)?,
        ))
    }
}
