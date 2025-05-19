use proc_macro_util::tokens::{Identifier, Literal};
use std::borrow::Cow;

mod new;
mod to_tokens;

/// Generates the tokens to display an enum variant
pub struct EnumVariantDisplay<'a> {
    /// The name of the variant
    name: Cow<'a, Identifier>,

    /// Does the variant have a field?
    has_field: bool,

    /// The literal which the variant matches on
    string: Literal,
}
