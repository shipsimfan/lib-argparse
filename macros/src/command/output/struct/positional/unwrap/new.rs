use super::PositionalUnwrap;
use proc_macro_util::tokens::Identifier;

impl PositionalUnwrap {
    /// Creates a new [`PositionalUnwrap`]
    pub fn new(variable_name: Identifier, info_name: Identifier) -> Self {
        PositionalUnwrap {
            variable_name,
            info_name,
        }
    }
}
