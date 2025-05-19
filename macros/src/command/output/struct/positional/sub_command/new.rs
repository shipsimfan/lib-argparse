use super::PositionalSubCommand;
use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

impl<'a> PositionalSubCommand<'a> {
    /// Creates a new [`PositionalSubCommand`]
    pub fn new(index: usize, variable_name: Cow<'a, Identifier>) -> Self {
        PositionalSubCommand {
            index,
            variable_name,
        }
    }
}
