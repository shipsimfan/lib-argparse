use proc_macro_util::tokens::Identifier;

mod new;
mod to_tokens;

/// Generates the tokens to display help for a positional
pub struct PositionalHelpOutput {
    /// The name of the info variable describing this positional
    info_name: Identifier,

    /// The offset for aligning descriptions
    description_offset: usize,
}
