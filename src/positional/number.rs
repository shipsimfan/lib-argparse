use crate::{Argument, Error, InvalidNumberError, Positional, PositionalInfo, PositionalResult};

macro_rules! impl_number {
    ($($t: ty),*) => {$(
        impl Positional for $t {
            fn parse(
                this: &mut Option<Self>,
                argument: Argument,
                info: &PositionalInfo<Self>,
            ) -> PositionalResult {
                match argument.as_str()?.parse() {
                    Ok(value) => *this = Some(value),
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
            fn parse(
                this: &mut Option<Self>,
                argument: Argument,
                info: &PositionalInfo<Self>,
            ) -> PositionalResult {
                match argument.as_str()?.parse() {
                    Ok(value) => *this = Some(<$t>::new(value)),
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
