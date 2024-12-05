use crate::{messages::errors::*, ArgumentSource, Error, Flag, FlagInfo, Result};
use i18n::translation::m;

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

impl From<std::num::ParseIntError> for InvalidNumberError {
    fn from(value: std::num::ParseIntError) -> Self {
        match *value.kind() {
            std::num::IntErrorKind::PosOverflow => InvalidNumberError::PosOverflow,
            std::num::IntErrorKind::NegOverflow => InvalidNumberError::NegOverflow,
            std::num::IntErrorKind::Zero => InvalidNumberError::Zero,
            _ => InvalidNumberError::Invalid,
        }
    }
}

impl From<std::num::ParseFloatError> for InvalidNumberError {
    fn from(_: std::num::ParseFloatError) -> Self {
        InvalidNumberError::Invalid
    }
}

macro_rules! impl_number {
    ($($t: ty),*) => {$(
        impl Flag for $t {
            fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
                let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

                value
                    .as_str()?
                    .parse()
                    .map_err(|error| Error::invalid_flag_value(info, long, Into::<InvalidNumberError>::into(error)))
            }
        }
    )*};
}

macro_rules! impl_atomic {
    ($($t: ty),*) => {$(
        impl Flag for $t {
            fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
                let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

                value
                    .as_str()?
                    .parse()
                    .map(<$t>::new)
                    .map_err(|error| Error::invalid_flag_value(info, long, Into::<InvalidNumberError>::into(error)))
            }
        }
    )*};
}

impl_number!(
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    std::num::NonZeroU8,
    std::num::NonZeroU16,
    std::num::NonZeroU32,
    std::num::NonZeroU64,
    std::num::NonZeroU128,
    std::num::NonZeroUsize,
    std::num::NonZeroI8,
    std::num::NonZeroI16,
    std::num::NonZeroI32,
    std::num::NonZeroI64,
    std::num::NonZeroI128,
    std::num::NonZeroIsize
);

impl_atomic!(
    std::sync::atomic::AtomicI8,
    std::sync::atomic::AtomicI16,
    std::sync::atomic::AtomicI32,
    std::sync::atomic::AtomicI64,
    std::sync::atomic::AtomicIsize,
    std::sync::atomic::AtomicU8,
    std::sync::atomic::AtomicU16,
    std::sync::atomic::AtomicU32,
    std::sync::atomic::AtomicU64,
    std::sync::atomic::AtomicUsize
);
