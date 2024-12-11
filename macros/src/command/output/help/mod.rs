mod description;
mod header;
mod name;
mod positional;
mod usage;

mod new;
mod to_tokens;

pub use description::HelpOutputDescription;
pub use header::HelpHeader;
pub use name::HelpOutputName;
pub use positional::PositionalHelpOutput;
pub use usage::{FlagHelpUsageOutput, HelpUsageOutput, PositionalHelpUsageOutput};

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
}
