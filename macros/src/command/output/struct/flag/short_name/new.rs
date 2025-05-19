use super::FlagShortName;
use proc_macro_util::tokens::{Identifier, Literal};
use std::borrow::Cow;

impl<'a> FlagShortName<'a> {
    /// Creates a new [`FlagShortName`]
    pub fn new(
        short_name: Literal,
        variable_name: Cow<'a, Identifier>,
        info_name: Identifier,
    ) -> Self {
        FlagShortName {
            short_name,
            variable_name,
            info_name,
        }
    }
}
