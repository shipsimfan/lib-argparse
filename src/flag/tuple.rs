use crate::{ArgumentSource, Error, Flag, FlagInfo, Result};

impl<T1: Flag, T2: Flag> Flag for (T1, T2) {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        if this.is_some() {
            return Err(Error::repeated_flag(info, long));
        }

        let mut new1 = None;
        let mut new2 = None;
        let info1 = info.drop_default();
        let info2 = info.drop_default();
        T1::parse(&mut new1, source, &info1, long)?;
        T2::parse(&mut new2, source, &info2, long)?;

        *this = Some((T1::unwrap(new1, &info1)?, T2::unwrap(new2, &info2)?));
        Ok(())
    }
}

impl<T1: Flag, T2: Flag, T3: Flag> Flag for (T1, T2, T3) {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        if this.is_some() {
            return Err(Error::repeated_flag(info, long));
        }

        let (mut new1, mut new2, mut new3) = (None, None, None);
        let info1 = info.drop_default();
        let info2 = info.drop_default();
        let info3 = info.drop_default();
        T1::parse(&mut new1, source, &info1, long)?;
        T2::parse(&mut new2, source, &info2, long)?;
        T3::parse(&mut new3, source, &info3, long)?;

        *this = Some((
            T1::unwrap(new1, &info1)?,
            T2::unwrap(new2, &info2)?,
            T3::unwrap(new3, &info3)?,
        ));
        Ok(())
    }
}

impl<T1: Flag, T2: Flag, T3: Flag, T4: Flag> Flag for (T1, T2, T3, T4) {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        if this.is_some() {
            return Err(Error::repeated_flag(info, long));
        }

        let (mut new1, mut new2, mut new3, mut new4) = (None, None, None, None);
        let info1 = info.drop_default();
        let info2 = info.drop_default();
        let info3 = info.drop_default();
        let info4 = info.drop_default();
        T1::parse(&mut new1, source, &info1, long)?;
        T2::parse(&mut new2, source, &info2, long)?;
        T3::parse(&mut new3, source, &info3, long)?;
        T4::parse(&mut new4, source, &info4, long)?;

        *this = Some((
            T1::unwrap(new1, &info1)?,
            T2::unwrap(new2, &info2)?,
            T3::unwrap(new3, &info3)?,
            T4::unwrap(new4, &info4)?,
        ));
        Ok(())
    }
}

impl<T1: Flag, T2: Flag, T3: Flag, T4: Flag, T5: Flag> Flag for (T1, T2, T3, T4, T5) {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        if this.is_some() {
            return Err(Error::repeated_flag(info, long));
        }

        let (mut new1, mut new2, mut new3, mut new4, mut new5) = (None, None, None, None, None);
        let info1 = info.drop_default();
        let info2 = info.drop_default();
        let info3 = info.drop_default();
        let info4 = info.drop_default();
        let info5 = info.drop_default();
        T1::parse(&mut new1, source, &info1, long)?;
        T2::parse(&mut new2, source, &info2, long)?;
        T3::parse(&mut new3, source, &info3, long)?;
        T4::parse(&mut new4, source, &info4, long)?;
        T5::parse(&mut new5, source, &info5, long)?;

        *this = Some((
            T1::unwrap(new1, &info1)?,
            T2::unwrap(new2, &info2)?,
            T3::unwrap(new3, &info3)?,
            T4::unwrap(new4, &info4)?,
            T5::unwrap(new5, &info5)?,
        ));
        Ok(())
    }
}

impl<T1: Flag, T2: Flag, T3: Flag, T4: Flag, T5: Flag, T6: Flag> Flag for (T1, T2, T3, T4, T5, T6) {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        if this.is_some() {
            return Err(Error::repeated_flag(info, long));
        }

        let (mut new1, mut new2, mut new3, mut new4, mut new5, mut new6) =
            (None, None, None, None, None, None);
        let info1 = info.drop_default();
        let info2 = info.drop_default();
        let info3 = info.drop_default();
        let info4 = info.drop_default();
        let info5 = info.drop_default();
        let info6 = info.drop_default();
        T1::parse(&mut new1, source, &info1, long)?;
        T2::parse(&mut new2, source, &info2, long)?;
        T3::parse(&mut new3, source, &info3, long)?;
        T4::parse(&mut new4, source, &info4, long)?;
        T5::parse(&mut new5, source, &info5, long)?;
        T6::parse(&mut new6, source, &info6, long)?;

        *this = Some((
            T1::unwrap(new1, &info1)?,
            T2::unwrap(new2, &info2)?,
            T3::unwrap(new3, &info3)?,
            T4::unwrap(new4, &info4)?,
            T5::unwrap(new5, &info5)?,
            T6::unwrap(new6, &info6)?,
        ));
        Ok(())
    }
}

impl<T1: Flag, T2: Flag, T3: Flag, T4: Flag, T5: Flag, T6: Flag, T7: Flag> Flag
    for (T1, T2, T3, T4, T5, T6, T7)
{
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        if this.is_some() {
            return Err(Error::repeated_flag(info, long));
        }

        let (mut new1, mut new2, mut new3, mut new4, mut new5, mut new6, mut new7) =
            (None, None, None, None, None, None, None);
        let info1 = info.drop_default();
        let info2 = info.drop_default();
        let info3 = info.drop_default();
        let info4 = info.drop_default();
        let info5 = info.drop_default();
        let info6 = info.drop_default();
        let info7 = info.drop_default();
        T1::parse(&mut new1, source, &info1, long)?;
        T2::parse(&mut new2, source, &info2, long)?;
        T3::parse(&mut new3, source, &info3, long)?;
        T4::parse(&mut new4, source, &info4, long)?;
        T5::parse(&mut new5, source, &info5, long)?;
        T6::parse(&mut new6, source, &info6, long)?;
        T7::parse(&mut new7, source, &info7, long)?;

        *this = Some((
            T1::unwrap(new1, &info1)?,
            T2::unwrap(new2, &info2)?,
            T3::unwrap(new3, &info3)?,
            T4::unwrap(new4, &info4)?,
            T5::unwrap(new5, &info5)?,
            T6::unwrap(new6, &info6)?,
            T7::unwrap(new7, &info7)?,
        ));
        Ok(())
    }
}

impl<T1: Flag, T2: Flag, T3: Flag, T4: Flag, T5: Flag, T6: Flag, T7: Flag, T8: Flag> Flag
    for (T1, T2, T3, T4, T5, T6, T7, T8)
{
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        if this.is_some() {
            return Err(Error::repeated_flag(info, long));
        }

        let (mut new1, mut new2, mut new3, mut new4, mut new5, mut new6, mut new7, mut new8) =
            (None, None, None, None, None, None, None, None);
        let info1 = info.drop_default();
        let info2 = info.drop_default();
        let info3 = info.drop_default();
        let info4 = info.drop_default();
        let info5 = info.drop_default();
        let info6 = info.drop_default();
        let info7 = info.drop_default();
        let info8 = info.drop_default();
        T1::parse(&mut new1, source, &info1, long)?;
        T2::parse(&mut new2, source, &info2, long)?;
        T3::parse(&mut new3, source, &info3, long)?;
        T4::parse(&mut new4, source, &info4, long)?;
        T5::parse(&mut new5, source, &info5, long)?;
        T6::parse(&mut new6, source, &info6, long)?;
        T7::parse(&mut new7, source, &info7, long)?;
        T8::parse(&mut new8, source, &info8, long)?;

        *this = Some((
            T1::unwrap(new1, &info1)?,
            T2::unwrap(new2, &info2)?,
            T3::unwrap(new3, &info3)?,
            T4::unwrap(new4, &info4)?,
            T5::unwrap(new5, &info5)?,
            T6::unwrap(new6, &info6)?,
            T7::unwrap(new7, &info7)?,
            T8::unwrap(new8, &info8)?,
        ));
        Ok(())
    }
}
