use super::Description;
use proc_macro_util::ast::Expression;

impl<'a> Description<'a> {
    /// Creates a new [`Description`] for `expression`
    pub fn new(expression: Vec<Expression<'a>>) -> Self {
        Description { expressions: expression }
    }
}
