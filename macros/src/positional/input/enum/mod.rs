use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;
use variant::EnumInputVariant;

mod variant;

mod extract;
mod into_output;

/// The details extracted from an enum
pub struct EnumInput<'a> {
    /// The name of the enum
    name: Cow<'a, Identifier>,

    /// The variants that make up this enum
    variants: Vec<EnumInputVariant<'a>>,
}
