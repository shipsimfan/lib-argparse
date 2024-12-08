use proc_macro_util::ast::Expression;

mod to_tokens;

/// The description to use when outputing the help flag
pub enum HelpOutputDescription<'a> {
    /// Use the description from the package
    Default,

    /// Use the provided description
    Provided(Expression<'a>),
}
