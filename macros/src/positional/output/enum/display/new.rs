use super::EnumVariantDisplay;
use proc_macro_util::tokens::{Identifier, Literal};
use std::borrow::Cow;

impl<'a> EnumVariantDisplay<'a> {
    /// Creates a new [`EnumVariantDisplay`]
    pub fn new(name: Cow<'a, Identifier>, has_field: bool, string: Literal) -> Self {
        EnumVariantDisplay {
            name,
            has_field,
            string,
        }
    }
}
