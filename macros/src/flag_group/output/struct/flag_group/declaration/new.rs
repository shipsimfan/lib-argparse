use super::FlagGroupDeclaration;
use proc_macro_util::ast::Type;

impl<'a> FlagGroupDeclaration<'a> {
    /// Creates a new [`FlagGroupDeclaration`] for `name`
    pub fn new(first: bool, r#type: Type<'a>) -> Self {
        FlagGroupDeclaration { first, r#type }
    }
}
