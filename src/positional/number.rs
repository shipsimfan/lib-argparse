use crate::{Argument, Error, InvalidNumberError, Positional, PositionalInfo, PositionalResult};

macro_rules! impl_number {
    ($($t: ty),*) => {$(
        impl Positional for $t {
            fn parse<'a>(
                this: &mut Option<Self>,
                argument: Argument<'a>,
                info: &PositionalInfo<Self>,
            ) -> PositionalResult<'a> {
                match argument.as_str()?.parse() {
                    Ok(value) => {
                        if let Some(min) = info.min {
                            if value < min as _ {
                                return PositionalResult::Error(Error::invalid_positional_value(info.value, InvalidNumberError::NegOverflow));
                            }
                        }

                        if let Some(max) = info.max {
                            if value > max as _ {
                                return PositionalResult::Error(Error::invalid_positional_value(info.value, InvalidNumberError::PosOverflow));
                            }
                        }

                        *this = Some(value)
                    },
                    Err(error) => {
                        return PositionalResult::Error(Error::invalid_positional_value(
                            info.value,
                            InvalidNumberError::from(error),
                        ))
                    }
                }
                PositionalResult::Next
            }
        }

        impl Positional for std::num::NonZero<$t> {
            fn parse<'a>(
                this: &mut Option<Self>,
                argument: Argument<'a>,
                info: &PositionalInfo<Self>,
            ) -> PositionalResult<'a> {
                match argument.as_str()?.parse::<std::num::NonZero<$t>>() {
                    Ok(value) => {
                        if let Some(min) = info.min {
                            if value.get() < min as _ {
                                return PositionalResult::Error(Error::invalid_positional_value(info.value, InvalidNumberError::NegOverflow));
                            }
                        }

                        if let Some(max) = info.max {
                            if value.get() > max as _ {
                                return PositionalResult::Error(Error::invalid_positional_value(info.value, InvalidNumberError::PosOverflow));
                            }
                        }

                        *this = Some(value)
                    },
                    Err(error) => {
                        return PositionalResult::Error(Error::invalid_positional_value(
                            info.value,
                            InvalidNumberError::from(error),
                        ))
                    }
                }
                PositionalResult::Next
            }
        }
    )*};
}

macro_rules! impl_atomic {
    ($($t: ty),*) => {$(
        impl Positional for $t {
            fn parse<'a>(
                this: &mut Option<Self>,
                argument: Argument<'a>,
                info: &PositionalInfo<Self>,
            ) -> PositionalResult<'a> {
                match argument.as_str()?.parse() {
                    Ok(value) => {
                        if let Some(min) = info.min {
                            if value < min as _ {
                                return PositionalResult::Error(Error::invalid_positional_value(info.value, InvalidNumberError::NegOverflow));
                            }
                        }

                        if let Some(max) = info.max {
                            if value > max as _ {
                                return PositionalResult::Error(Error::invalid_positional_value(info.value, InvalidNumberError::PosOverflow));
                            }
                        }

                        *this = Some(<$t>::new(value))
                    },
                    Err(error) => {
                        return PositionalResult::Error(Error::invalid_positional_value(
                            info.value,
                            InvalidNumberError::from(error),
                        ))
                    }
                }
                PositionalResult::Next
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

impl Positional for f32 {
    fn parse<'a>(
        this: &mut Option<Self>,
        argument: Argument<'a>,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult<'a> {
        match argument.as_str()?.parse() {
            Ok(value) => {
                if let Some(min) = info.min {
                    if value < min as _ {
                        return PositionalResult::Error(Error::invalid_positional_value(
                            info.value,
                            InvalidNumberError::NegOverflow,
                        ));
                    }
                }

                if let Some(max) = info.max {
                    if value > max as _ {
                        return PositionalResult::Error(Error::invalid_positional_value(
                            info.value,
                            InvalidNumberError::PosOverflow,
                        ));
                    }
                }

                *this = Some(value)
            }
            Err(error) => {
                return PositionalResult::Error(Error::invalid_positional_value(
                    info.value,
                    InvalidNumberError::from(error),
                ))
            }
        }
        PositionalResult::Next
    }
}

impl Positional for f64 {
    fn parse<'a>(
        this: &mut Option<Self>,
        argument: Argument<'a>,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult<'a> {
        match argument.as_str()?.parse() {
            Ok(value) => {
                if let Some(min) = info.min {
                    if value < min as _ {
                        return PositionalResult::Error(Error::invalid_positional_value(
                            info.value,
                            InvalidNumberError::NegOverflow,
                        ));
                    }
                }

                if let Some(max) = info.max {
                    if value > max as _ {
                        return PositionalResult::Error(Error::invalid_positional_value(
                            info.value,
                            InvalidNumberError::PosOverflow,
                        ));
                    }
                }

                *this = Some(value)
            }
            Err(error) => {
                return PositionalResult::Error(Error::invalid_positional_value(
                    info.value,
                    InvalidNumberError::from(error),
                ))
            }
        }
        PositionalResult::Next
    }
}
