use proc_macro_util::ast::Expression;

mod to_tokens;

/// The header for a section
pub enum HelpHeader<'a> {
    /// The user provided a header
    UserProvided(Expression<'a>),

    /// Use the default header
    Default(&'static str),
}
