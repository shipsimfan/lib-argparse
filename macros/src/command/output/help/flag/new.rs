use super::FlagHelpOutput;
use proc_macro_util::tokens::Identifier;

impl FlagHelpOutput {
    /// Creates a new [`FlagHelpOutput`]
    pub fn new(info_name: Identifier, description_offset: usize, short_names: bool) -> Self {
        FlagHelpOutput {
            info_name,
            description_offset,
            short_names,
        }
    }
}
