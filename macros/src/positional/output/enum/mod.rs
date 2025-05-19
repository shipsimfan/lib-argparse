use proc_macro_util::tokens::{Identifier, Literal};
use std::borrow::Cow;

mod display;
mod parse;
mod sub;

mod new;
mod to_tokens;

pub use display::EnumVariantDisplay;
pub use parse::EnumVariantParse;
pub use sub::EnumVariantSub;

/// The output code for an enum
pub struct EnumOutput<'a> {
    /// The name of the enum
    name: Cow<'a, Identifier>,

    /// The match arms to parse the provided argument
    parses: Vec<EnumVariantParse<'a>>,

    /// The string describing the expected values
    expected: Literal,

    /// Produces sub-commands if a variant with one is matched
    subs: Vec<EnumVariantSub<'a>>,

    /// The match arms of the std::fmt::Display implementation
    displays: Vec<EnumVariantDisplay<'a>>,
}
