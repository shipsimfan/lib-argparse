use super::FlagGroupUnwrap;
use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

impl<'a> FlagGroupUnwrap<'a> {
    /// Creates a new [`FlagGroupUnwrap`]
    pub fn new(variable_name: Cow<'a, Identifier>, index: usize) -> Self {
        FlagGroupUnwrap {
            variable_name,
            index,
        }
    }
}
