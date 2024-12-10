use super::FlagHelpUsageOutput;
use proc_macro_util::tokens::Identifier;

impl FlagHelpUsageOutput {
    /// Creates a new [`FlagHelpUsageOutput`]
    pub fn new(info_name: Identifier) -> Self {
        FlagHelpUsageOutput { info_name }
    }
}
