use proc_macro_util::ToTokens;

mod to_tokens;

/// A value which should be wrapped in an [`Option`]
pub enum OptionalOutput<T: ToTokens> {
    /// There is no value
    None,

    /// There is a value
    Some(T),
}
