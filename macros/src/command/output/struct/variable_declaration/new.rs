use super::VariableDeclaration;
use proc_macro_util::tokens::Identifier;

impl VariableDeclaration {
    /// Creates a new [`VariableDeclaration`] for `name`
    pub fn new(variable_name: Identifier) -> Self {
        VariableDeclaration { variable_name }
    }
}
