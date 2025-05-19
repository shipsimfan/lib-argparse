use super::{EnumOutput, EnumVariantDisplay, EnumVariantParse, EnumVariantSub};
use proc_macro_util::tokens::{Identifier, Literal};
use std::borrow::Cow;

impl<'a> EnumOutput<'a> {
    /// Creates a new [`EnumOutput`]
    pub fn new(
        name: Cow<'a, Identifier>,
        parses: Vec<EnumVariantParse<'a>>,
        expected: Literal,
        subs: Vec<EnumVariantSub<'a>>,
        displays: Vec<EnumVariantDisplay<'a>>,
    ) -> Self {
        EnumOutput {
            name,
            parses,
            expected,
            subs,
            displays,
        }
    }
}
