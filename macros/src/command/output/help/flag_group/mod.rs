use proc_macro_util::{ast::Type, tokens::Literal};

mod new;
mod to_tokens;

/// Generates the usage string for a flag
pub struct FlagGroupHelpOutput<'a> {
    /// The header for the group
    header: Literal,

    /// The type of the flag group
    r#type: Type<'a>,
}
