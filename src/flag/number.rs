use crate::{ArgumentSource, Error, Flag, FlagInfo, Result};
use std::{
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    sync::atomic::{
        AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16, AtomicU32, AtomicU64,
        AtomicU8, AtomicUsize,
    },
};

impl Flag for i8 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for i16 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for i32 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for i64 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for i128 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for isize {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for u8 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for u16 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for u32 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for u64 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for u128 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for usize {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for f32 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for f64 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for AtomicI8 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map(AtomicI8::new)
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for AtomicI16 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map(AtomicI16::new)
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for AtomicI32 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map(AtomicI32::new)
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for AtomicI64 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map(AtomicI64::new)
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for AtomicIsize {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map(AtomicIsize::new)
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for AtomicU8 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map(AtomicU8::new)
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for AtomicU16 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map(AtomicU16::new)
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for AtomicU32 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map(AtomicU32::new)
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for AtomicU64 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map(AtomicU64::new)
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for AtomicUsize {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map(AtomicUsize::new)
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for NonZeroI8 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for NonZeroI16 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for NonZeroI32 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for NonZeroI64 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for NonZeroI128 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for NonZeroIsize {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for NonZeroU8 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for NonZeroU16 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for NonZeroU32 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for NonZeroU64 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for NonZeroU128 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}

impl Flag for NonZeroUsize {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, error))
    }
}
