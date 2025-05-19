use super::FlagGroupDeclaration;
use proc_macro_util::{ast::Type, tokens::Identifier};
use std::borrow::Cow;

impl<'a> FlagGroupDeclaration<'a> {
    /// Creates a new [`FlagGroupDeclaration`] for `name`
    pub fn new(variable_name: Cow<'a, Identifier>, r#type: Type<'a>) -> Self {
        FlagGroupDeclaration {
            variable_name,
            r#type,
        }
    }
}
