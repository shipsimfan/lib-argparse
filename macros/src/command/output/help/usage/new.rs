use super::{FlagHelpUsageOutput, HelpUsageOutput, PositionalHelpUsageOutput};
use crate::command::output::help::HelpHeader;

impl<'a> HelpUsageOutput<'a> {
    /// Creates a new [`HelpUsageOutput`]
    pub fn new(
        header: HelpHeader<'a>,
        positionals: Vec<PositionalHelpUsageOutput>,
        flags: Vec<FlagHelpUsageOutput>,
    ) -> Self {
        HelpUsageOutput {
            header,
            positionals,
            flags,
        }
    }
}
