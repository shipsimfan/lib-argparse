use proc_macro_util::ToTokens;

mod new;
mod to_tokens;

/// Outputs a value converted to an f64
pub struct AsF64<T: ToTokens> {
    /// The value to convert
    pub value: T,
}
