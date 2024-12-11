use super::PositionalHelpOutput;
use proc_macro_util::tokens::Identifier;

impl PositionalHelpOutput {
    /// Creates a new [`PositionalHelpOutput`]
    pub fn new(info_name: Identifier, description_offset: usize) -> Self {
        PositionalHelpOutput {
            info_name,
            description_offset,
        }
    }
}
