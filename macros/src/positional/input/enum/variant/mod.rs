use proc_macro_util::{ast::Type, tokens::Identifier};
use std::borrow::Cow;

mod extract;
mod into_output;

/// The details extracted from an enum variant
pub struct EnumInputVariant<'a> {
    /// The name of the variant
    name: Cow<'a, Identifier>,

    /// The type of the field associated with this variant, if there is one
    r#type: Option<Type<'a>>,
}
