use proc_macro_util::tokens::{Identifier, Literal};
use std::borrow::Cow;

mod new;
mod to_tokens;

/// Generates the match arm for a flag's long name
pub struct FlagLongName<'a> {
    /// The long name of the flag
    long_name: Literal,

    /// The name of this flag's variable
    variable_name: Cow<'a, Identifier>,

    /// The name of the info describing this flag
    info_name: Identifier,
}
