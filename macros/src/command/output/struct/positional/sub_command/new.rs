use super::PositionalSubCommand;
use proc_macro_util::tokens::Identifier;

impl PositionalSubCommand {
    /// Creates a new [`PositionalSubCommand`]
    pub fn new(index: usize, variable_name: Identifier) -> Self {
        PositionalSubCommand {
            index,
            variable_name,
        }
    }
}
