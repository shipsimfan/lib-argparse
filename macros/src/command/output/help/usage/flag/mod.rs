use proc_macro_util::tokens::Identifier;

mod new;
mod to_tokens;

/// Generates the usage string for a flag
pub struct FlagHelpUsageOutput {
    /// The name for the information describing this flag
    info_name: Identifier,
}
