use proc_macro_util::ast::Expression;

mod to_tokens;

/// A default value for a flag or positional
pub enum DefaultValue<'a> {
    /// There is no default value
    None,

    /// There is a default value
    Some(Expression<'a>),
}
