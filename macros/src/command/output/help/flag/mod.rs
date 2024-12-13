use proc_macro_util::tokens::Identifier;

mod new;
mod to_tokens;

/// Generates the tokens to display help for a flag
pub struct FlagHelpOutput {
    /// The name of the info variable describing this flag
    info_name: Identifier,

    /// The offset for aligning descriptions
    description_offset: usize,

    /// Do any flags have short names?
    short_names: bool,
}
