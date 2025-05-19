use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

mod new;
mod to_tokens;

/// Unwraps a positional variable
pub struct PositionalUnwrap<'a> {
    /// The name of the variable to unwrap
    variable_name: Cow<'a, Identifier>,

    /// The name of the information describing the positional
    info_name: Identifier,
}
