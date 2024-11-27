use proc_macro_util::tokens::Identifier;

mod new;
mod to_tokens;

/// Generates the calling for a positional's `sub` function
pub struct PositionalSubCommand {
    /// The index of this positional
    index: usize,

    /// The name of the variable for this positional
    variable_name: Identifier,
}
