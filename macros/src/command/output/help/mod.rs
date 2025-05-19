mod description;
mod flag;
mod flag_group;
mod header;
mod name;
mod positional;
mod usage;

mod new;
mod to_tokens;

pub use description::HelpOutputDescription;
pub use flag::FlagHelpOutput;
pub use flag_group::FlagGroupHelpOutput;
pub use header::HelpHeader;
pub use name::HelpOutputName;
pub use positional::PositionalHelpOutput;
pub use usage::{
    FlagGroupHelpUsageOutput, FlagHelpUsageOutput, HelpUsageOutput, PositionalHelpUsageOutput,
};

/// Generates the code to display a help message
pub struct HelpOutput<'a> {
    /// The name of the program to use
    name: HelpOutputName,

    /// The description to use
    description: HelpOutputDescription<'a>,

    /// The usage to output with this help
    usage: HelpUsageOutput<'a>,

    /// The header for the positional section
    positional_header: Option<HelpHeader<'a>>,

    /// The positionals to display help for
    positionals: Vec<PositionalHelpOutput>,

    /// The header for the flags section
    flag_header: Option<HelpHeader<'a>>,

    /// The flags to display help for
    flags: Vec<FlagHelpOutput>,

    /// The flag groups to display help for
    flag_groups: Vec<FlagGroupHelpOutput<'a>>,
}
