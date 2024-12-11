use super::HelpHeader;

mod flag;
mod positional;

mod new;
mod to_tokens;

pub use flag::FlagHelpUsageOutput;
pub use positional::PositionalHelpUsageOutput;

/// Produces the usage for a help message
pub struct HelpUsageOutput<'a> {
    /// The header for the usage
    header: HelpHeader<'a>,

    /// The positional usages
    positionals: Vec<PositionalHelpUsageOutput>,

    /// The flag usages
    flags: Vec<FlagHelpUsageOutput>,
}
