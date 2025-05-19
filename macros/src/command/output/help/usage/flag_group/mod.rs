use proc_macro_util::ast::Type;

mod new;
mod to_tokens;

/// Generates the usage string for a flag
pub struct FlagGroupHelpUsageOutput<'a> {
    /// The type of the flag group
    r#type: Type<'a>,
}
