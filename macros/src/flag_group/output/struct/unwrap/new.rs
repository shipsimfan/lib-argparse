use super::FlagUnwrap;
use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

impl<'a> FlagUnwrap<'a> {
    /// Creates a new [`FlagUnwrap`]
    pub fn new(variable_name: Cow<'a, Identifier>, index: usize, info_name: Identifier) -> Self {
        FlagUnwrap {
            variable_name,
            index,
            info_name,
        }
    }
}
