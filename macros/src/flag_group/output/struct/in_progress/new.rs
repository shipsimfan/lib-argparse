use super::InProgress;
use proc_macro_util::ast::Type;

impl<'a> InProgress<'a> {
    /// Creates a new [`InProgress`] from `types`
    pub fn new(types: Vec<Type<'a>>) -> Self {
        InProgress { types }
    }
}
