use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

mod new;
mod to_tokens;

/// Unwraps a flag variable
pub struct FlagGroupUnwrap<'a> {
    /// The name of the variable to unwrap
    variable_name: Cow<'a, Identifier>,
}
