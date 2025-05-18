use super::AsF64;
use proc_macro_util::ToTokens;

impl<T: ToTokens> AsF64<T> {
    /// Creates a new [`AsF64`]
    pub fn new(value: T) -> Self {
        AsF64 { value }
    }
}
