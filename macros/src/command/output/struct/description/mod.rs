use proc_macro_util::ast::Expression;

mod new;
mod to_tokens;

/// The description for a flag or positional
pub struct Description<'a> {
    /// The expression defining the description
    expression: Expression<'a>,
}
