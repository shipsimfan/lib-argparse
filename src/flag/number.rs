use crate::{messages::errors::*, ArgumentSource, Error, Flag, FlagInfo, Result};
use i18n::translation::m;
use std::{
    num::{
        IntErrorKind, NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize,
        NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, ParseIntError,
    },
    sync::atomic::{
        AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16, AtomicU32, AtomicU64,
        AtomicU8, AtomicUsize,
    },
};

/// An invalid number was parsed
#[derive(Debug)]
pub enum InvalidNumberError {
    /// The value provided was not a valid number
    Invalid,

    /// The value provided is too large
    PosOverflow,

    /// The value provided is too small
    NegOverflow,

    /// The value provided is zero when it shouldn't be
    Zero,
}

impl InvalidNumberError {
    /// Creates a new [`InvalidNumberError`]
    pub(self) fn new(error: ParseIntError) -> Self {
        error.into()
    }
}

impl std::error::Error for InvalidNumberError {}

impl std::fmt::Display for InvalidNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidNumberError::Invalid => m!(NumberInvalid).fmt(f),
            InvalidNumberError::PosOverflow => m!(NumberPosOverflow).fmt(f),
            InvalidNumberError::NegOverflow => m!(NumberNegOverflow).fmt(f),
            InvalidNumberError::Zero => m!(NumberZero).fmt(f),
        }
    }
}

impl From<ParseIntError> for InvalidNumberError {
    fn from(value: ParseIntError) -> Self {
        match *value.kind() {
            IntErrorKind::PosOverflow => InvalidNumberError::PosOverflow,
            IntErrorKind::NegOverflow => InvalidNumberError::NegOverflow,
            IntErrorKind::Zero => InvalidNumberError::Zero,
            _ => InvalidNumberError::Invalid,
        }
    }
}

impl Flag for i8 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for i16 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for i32 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for i64 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for i128 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for isize {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for u8 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for u16 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for u32 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for u64 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for u128 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for usize {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
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
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for AtomicI16 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map(AtomicI16::new)
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for AtomicI32 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map(AtomicI32::new)
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for AtomicI64 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map(AtomicI64::new)
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for AtomicIsize {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map(AtomicIsize::new)
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for AtomicU8 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map(AtomicU8::new)
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for AtomicU16 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map(AtomicU16::new)
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for AtomicU32 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map(AtomicU32::new)
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for AtomicU64 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map(AtomicU64::new)
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for AtomicUsize {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map(AtomicUsize::new)
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for NonZeroI8 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for NonZeroI16 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for NonZeroI32 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for NonZeroI64 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for NonZeroI128 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for NonZeroIsize {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for NonZeroU8 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for NonZeroU16 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for NonZeroU32 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for NonZeroU64 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for NonZeroU128 {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}

impl Flag for NonZeroUsize {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        value
            .as_str()?
            .parse()
            .map_err(|error| Error::invalid_flag_value(info, long, InvalidNumberError::new(error)))
    }
}
