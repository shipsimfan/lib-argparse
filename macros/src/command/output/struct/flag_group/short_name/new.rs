use super::FlagGroupShortName;
use proc_macro_util::{ast::Type, tokens::Identifier};
use std::borrow::Cow;

impl<'a> FlagGroupShortName<'a> {
    /// Creates a new [`FlagLongName`]
    pub fn new(variable_name: Cow<'a, Identifier>, r#type: Type<'a>) -> Self {
        FlagGroupShortName {
            variable_name,
            r#type,
        }
    }
}
