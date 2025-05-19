use super::FlagGroupLongName;
use proc_macro_util::{ast::Type, tokens::Identifier};
use std::borrow::Cow;

impl<'a> FlagGroupLongName<'a> {
    /// Creates a new [`FlagLongName`]
    pub fn new(variable_name: Cow<'a, Identifier>, r#type: Type<'a>) -> Self {
        FlagGroupLongName {
            variable_name,
            r#type,
        }
    }
}
