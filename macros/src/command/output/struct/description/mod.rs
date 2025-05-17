use proc_macro_util::ast::Expression;

mod new;
mod to_tokens;

/// The description for a flag or positional
pub struct Description<'a> {
    /// The expressions defining the description
    expressions: Vec<Expression<'a>>,
}
