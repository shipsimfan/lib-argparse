use std::borrow::Cow;

use super::VariableDeclaration;
use proc_macro_util::tokens::Identifier;

impl<'a> VariableDeclaration<'a> {
    /// Creates a new [`VariableDeclaration`] for `name`
    pub fn new(variable_name: Cow<'a, Identifier>) -> Self {
        VariableDeclaration { variable_name }
    }
}
