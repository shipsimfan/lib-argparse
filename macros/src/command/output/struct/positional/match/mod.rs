use proc_macro_util::tokens::Identifier;

mod new;
mod to_tokens;

/// Generates the match arm for a positional
pub struct PositionalMatch {
    /// The index of this positional
    index: usize,

    /// The name of the variable for this positional
    variable_name: Identifier,

    /// The name of info describing this positional
    info_name: Identifier,
}
