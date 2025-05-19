use super::EnumVariantSub;
use proc_macro_util::{
    ast::Type,
    tokens::{Identifier, Literal},
};
use std::borrow::Cow;

impl<'a> EnumVariantSub<'a> {
    /// Creates a new [`EnumVariantSub`]
    pub fn new(string: Literal, r#type: Type<'a>, name: Cow<'a, Identifier>) -> Self {
        EnumVariantSub {
            string,
            r#type,
            name,
        }
    }
}
