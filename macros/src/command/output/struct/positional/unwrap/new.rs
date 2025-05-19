use super::PositionalUnwrap;
use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

impl<'a> PositionalUnwrap<'a> {
    /// Creates a new [`PositionalUnwrap`]
    pub fn new(variable_name: Cow<'a, Identifier>, info_name: Identifier) -> Self {
        PositionalUnwrap {
            variable_name,
            info_name,
        }
    }
}
