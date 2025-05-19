use super::PositionalMatch;
use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

impl<'a> PositionalMatch<'a> {
    /// Creates a new [`PositionalMatch`]
    pub fn new(index: usize, variable_name: Cow<'a, Identifier>, info_name: Identifier) -> Self {
        PositionalMatch {
            index,
            variable_name,
            info_name,
        }
    }
}
