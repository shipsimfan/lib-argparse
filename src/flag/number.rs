use crate::{ArgumentSource, Error, Flag, FlagInfo, InvalidNumberError, Result};

macro_rules! impl_number {
    ($($t: ty),*) => {$(
        impl Flag for $t {
            fn parse(
                this: &mut Option<Self>,
                source: &mut dyn ArgumentSource,
                info: &FlagInfo<Self>,
                long: bool
            ) -> Result<()> {
                if this.is_some() {
                    return Err(Error::repeated_flag(info, long));
                }

                let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

                let value = value
                    .as_str()?
                    .parse()
                    .map_err(|error| Error::invalid_flag_value(info, long, Into::<InvalidNumberError>::into(error)))?;

                if let Some(min) = info.min {
                    if value < min as _ {
                        return Err(Error::invalid_flag_value(info, long, InvalidNumberError::NegOverflow));
                    }
                }

                if let Some(max) = info.max {
                    if value > max as _ {
                        return Err(Error::invalid_flag_value(info, long, InvalidNumberError::PosOverflow));
                    }
                }

                *this = Some(value);
                Ok(())
            }
        }

        impl Flag for std::num::NonZero<$t> {
            fn parse(
                this: &mut Option<Self>,
                source: &mut dyn ArgumentSource,
                info: &FlagInfo<Self>,
                long: bool
            ) -> Result<()> {
                let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

                let value: std::num::NonZero<$t> = value
                    .as_str()?
                    .parse()
                    .map_err(|error| Error::invalid_flag_value(info, long, Into::<InvalidNumberError>::into(error)))?;

                if let Some(min) = info.min {
                    if value.get() < min as _ {
                        return Err(Error::invalid_flag_value(info, long, InvalidNumberError::NegOverflow));
                    }
                }

                if let Some(max) = info.max {
                    if value.get() > max as _ {
                        return Err(Error::invalid_flag_value(info, long, InvalidNumberError::PosOverflow));
                    }
                }

                *this = Some(value);
                Ok(())
            }
        }
    )*};
}

macro_rules! impl_atomic {
    ($($t: ty),*) => {$(
        impl Flag for $t {
            fn parse(
                this: &mut Option<Self>,
                source: &mut dyn ArgumentSource,
                info: &FlagInfo<Self>,
                long: bool
            ) -> Result<()> {
                let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

               let value = value
                    .as_str()?
                    .parse()
                    .map_err(|error| Error::invalid_flag_value(info, long, Into::<InvalidNumberError>::into(error)))?;

                if let Some(min) = info.min {
                    if value < min as _ {
                        return Err(Error::invalid_flag_value(info, long, InvalidNumberError::NegOverflow));
                    }
                }

                if let Some(max) = info.max {
                    if value > max as _ {
                        return Err(Error::invalid_flag_value(info, long, InvalidNumberError::PosOverflow));
                    }
                }

                *this = Some(<$t>::new(value));
                Ok(())
            }
        }
    )*};
}

impl_number!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

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

impl Flag for f32 {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        let value = value.as_str()?.parse().map_err(|error| {
            Error::invalid_flag_value(info, long, Into::<InvalidNumberError>::into(error))
        })?;

        if let Some(min) = info.min {
            if value < min as _ {
                return Err(Error::invalid_flag_value(
                    info,
                    long,
                    InvalidNumberError::NegOverflow,
                ));
            }
        }

        if let Some(max) = info.max {
            if value > max as _ {
                return Err(Error::invalid_flag_value(
                    info,
                    long,
                    InvalidNumberError::PosOverflow,
                ));
            }
        }

        *this = Some(value);
        Ok(())
    }
}

impl Flag for f64 {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        let value = source.next().ok_or(Error::missing_flag_value(info, long))?;

        let value = value.as_str()?.parse().map_err(|error| {
            Error::invalid_flag_value(info, long, Into::<InvalidNumberError>::into(error))
        })?;

        if let Some(min) = info.min {
            if value < min {
                return Err(Error::invalid_flag_value(
                    info,
                    long,
                    InvalidNumberError::NegOverflow,
                ));
            }
        }

        if let Some(max) = info.max {
            if value > max {
                return Err(Error::invalid_flag_value(
                    info,
                    long,
                    InvalidNumberError::PosOverflow,
                ));
            }
        }

        *this = Some(value);
        Ok(())
    }
}
