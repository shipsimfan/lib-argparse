use proc_macro_util::{ast::Expression, tokens::Literal};

mod to_tokens;

/// Generates the code to display version
pub enum VersionOutput<'a> {
    /// The user provided an expression to display
    UserDefined(Expression<'a>),

    /// The user didn't provide an expression, but did provide a name
    AlternateName(Literal),

    /// The user didn't provide either an expression or a name
    Default,
}
