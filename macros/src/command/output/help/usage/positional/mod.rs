use proc_macro_util::tokens::{Identifier, Literal};

mod new;
mod to_tokens;

/// Generates the usage string for a positional
pub struct PositionalHelpUsageOutput {
    /// The name for the information describing this positional
    info_name: Identifier,

    /// The name for the value
    value: Literal,
}
