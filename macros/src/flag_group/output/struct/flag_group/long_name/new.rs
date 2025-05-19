use super::FlagGroupLongName;
use proc_macro_util::ast::Type;

impl<'a> FlagGroupLongName<'a> {
    /// Creates a new [`FlagLongName`]
    pub fn new(index: usize, r#type: Type<'a>) -> Self {
        FlagGroupLongName { index, r#type }
    }
}
