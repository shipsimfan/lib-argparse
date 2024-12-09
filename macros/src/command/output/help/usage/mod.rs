use proc_macro_util::ast::Expression;

mod new;
mod positional;
mod to_tokens;

pub use positional::PositionalHelpUsageOutput;

/// Produces the usage for a help message
pub struct HelpUsageOutput<'a> {
    /// The header for the usage
    header: Option<Expression<'a>>,

    /// The positional usages
    positionals: Vec<PositionalHelpUsageOutput>,
}
