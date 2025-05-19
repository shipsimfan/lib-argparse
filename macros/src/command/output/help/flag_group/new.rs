use super::FlagGroupHelpOutput;
use proc_macro_util::{ast::Type, tokens::Literal};

impl<'a> FlagGroupHelpOutput<'a> {
    /// Creates a new [`FlagGroupHelpOutput`]
    pub fn new(header: Literal, r#type: Type<'a>) -> Self {
        FlagGroupHelpOutput { header, r#type }
    }
}
