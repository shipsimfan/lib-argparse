use proc_macro_util::ast::Expression;

mod new;
mod to_tokens;

/// The default value for a flag or positional
pub struct DefaultValue<'a> {
    /// The expression defining the default value
    expression: Expression<'a>,
}
