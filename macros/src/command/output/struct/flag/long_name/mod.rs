use proc_macro_util::tokens::{Identifier, Literal};

mod new;
mod to_tokens;

/// Generates the match arm for this flags long name
pub struct FlagLongName {
    /// The long name of the flag
    long_name: Literal,

    /// The name of this flag's variable
    variable_name: Identifier,

    /// The name of the info describing this flag
    info_name: Identifier,
}
