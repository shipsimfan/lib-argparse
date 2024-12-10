use super::PositionalHelpUsageOutput;
use proc_macro_util::tokens::Identifier;

impl PositionalHelpUsageOutput {
    /// Creates a new [`PositionalHelpUsageOutput`]
    pub fn new(info_name: Identifier) -> Self {
        PositionalHelpUsageOutput { info_name }
    }
}
