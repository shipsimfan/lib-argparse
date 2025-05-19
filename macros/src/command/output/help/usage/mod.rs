use super::HelpHeader;

mod flag;
mod flag_group;
mod positional;

mod new;
mod to_tokens;

pub use flag::FlagHelpUsageOutput;
pub use flag_group::FlagGroupHelpUsageOutput;
pub use positional::PositionalHelpUsageOutput;

/// Produces the usage for a help message
pub struct HelpUsageOutput<'a> {
    /// The header for the usage
    header: HelpHeader<'a>,

    /// The positional usages
    positionals: Vec<PositionalHelpUsageOutput>,

    /// The flag usages
    flags: Vec<FlagHelpUsageOutput>,

    /// The flag group usages
    flag_groups: Vec<FlagGroupHelpUsageOutput<'a>>,
}
