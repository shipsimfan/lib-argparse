use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

mod new;
mod to_tokens;

/// Unwraps a flag variable
pub struct FlagUnwrap<'a> {
    /// The name of the variable to unwrap
    variable_name: Cow<'a, Identifier>,

    /// The index in the tuple of this variable
    index: usize,

    /// The name of the information describing the flag
    info_name: Identifier,
}
