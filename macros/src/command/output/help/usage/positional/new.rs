use super::PositionalHelpUsageOutput;
use proc_macro_util::tokens::{Identifier, Literal};

impl PositionalHelpUsageOutput {
    /// Creates a new [`PositionalHelpUsageOutput`]
    pub fn new(info_name: Identifier, value: Literal) -> Self {
        PositionalHelpUsageOutput { info_name, value }
    }
}
