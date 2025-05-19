use proc_macro_util::tokens::{Identifier, Literal};
use std::borrow::Cow;

mod new;
mod to_tokens;

/// Generates the tokens to parse an enum variant
pub struct EnumVariantParse<'a> {
    /// The literal which the variant matches on
    string: Literal,

    /// The name of the variant, or [`None`] if this variant produces a sub-command
    name: Option<Cow<'a, Identifier>>,
}
