use crate::DefaultDisplay;

macro_rules! impl_display {
    ($($t: ty),*) => {$(
        impl DefaultDisplay for $t {
            type Display<'a> = &'a Self;

            fn as_display<'a>(&'a self) -> Self::Display<'a> {
                self
            }
        }
    )*};
}

impl_display!(
    bool,
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
    String,
    std::num::NonZeroI8,
    std::num::NonZeroI16,
    std::num::NonZeroI32,
    std::num::NonZeroI64,
    std::num::NonZeroI128,
    std::num::NonZeroIsize,
    std::num::NonZeroU8,
    std::num::NonZeroU16,
    std::num::NonZeroU32,
    std::num::NonZeroU64,
    std::num::NonZeroU128,
    std::num::NonZeroUsize
);
