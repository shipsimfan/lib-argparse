use super::FlagGroupShortName;
use proc_macro_util::ast::Type;

impl<'a> FlagGroupShortName<'a> {
    /// Creates a new [`FlagGroupShortName`]
    pub fn new(index: usize, r#type: Type<'a>) -> Self {
        FlagGroupShortName { index, r#type }
    }
}
