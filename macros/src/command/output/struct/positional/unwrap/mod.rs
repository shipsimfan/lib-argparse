use proc_macro_util::tokens::Identifier;

mod new;
mod to_tokens;

/// Unwraps a positional variable
pub struct PositionalUnwrap {
    /// The name of the variable to unwrap
    variable_name: Identifier,

    /// The name of the information describing the positional
    info_name: Identifier,
}
