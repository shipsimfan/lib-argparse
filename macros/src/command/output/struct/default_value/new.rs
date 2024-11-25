use super::DefaultValue;
use proc_macro_util::ast::Expression;

impl<'a> DefaultValue<'a> {
    /// Creates a new [`DefaultValue`] for `expression`
    pub fn new(expression: Expression<'a>) -> Self {
        DefaultValue { expression }
    }
}
