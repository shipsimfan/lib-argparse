use super::PositionalMatch;
use proc_macro_util::tokens::Identifier;

impl PositionalMatch {
    /// Creates a new [`PositionalMatch`]
    pub fn new(index: usize, variable_name: Identifier, info_name: Identifier) -> Self {
        PositionalMatch {
            index,
            variable_name,
            info_name,
        }
    }
}
