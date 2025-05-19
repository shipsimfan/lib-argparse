use super::FlagGroupInProgress;
use proc_macro_util::ast::Type;

impl<'a> FlagGroupInProgress<'a> {
    /// Creates a new [`FlagGroupInProgress`] for `name`
    pub fn new(first: bool, r#type: Type<'a>) -> Self {
        FlagGroupInProgress { first, r#type }
    }
}
