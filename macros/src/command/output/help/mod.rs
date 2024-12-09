mod description;
mod name;
mod usage;

mod new;
mod to_tokens;

pub use description::HelpOutputDescription;
pub use name::HelpOutputName;
pub use usage::{HelpUsageOutput, PositionalHelpUsageOutput};

/// Generates the code to display a help message
pub struct HelpOutput<'a> {
    /// The name of the program to use
    name: HelpOutputName,

    /// The description to use
    description: HelpOutputDescription<'a>,

    /// The usage to output with this help
    usage: HelpUsageOutput<'a>,
}
