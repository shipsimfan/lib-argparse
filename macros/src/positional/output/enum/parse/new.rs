use super::EnumVariantParse;
use proc_macro_util::tokens::{Identifier, Literal};
use std::borrow::Cow;

impl<'a> EnumVariantParse<'a> {
    /// Creates a new [`EnumVariantParse`]
    pub fn new(string: Literal, name: Option<Cow<'a, Identifier>>) -> Self {
        EnumVariantParse { string, name }
    }
}
