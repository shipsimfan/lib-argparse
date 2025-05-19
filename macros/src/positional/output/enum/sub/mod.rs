use proc_macro_util::{
    ast::Type,
    tokens::{Identifier, Literal},
};
use std::borrow::Cow;

mod new;
mod to_tokens;

/// Generates the tokens to start the sub-command of an enum variant
pub struct EnumVariantSub<'a> {
    /// The literal which the variant matches on
    string: Literal,

    /// The type of the sub-command
    r#type: Type<'a>,

    /// The name of the variant to produce
    name: Cow<'a, Identifier>,
}
