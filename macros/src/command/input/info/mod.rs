use proc_macro_util::{ast::Expression, tokens::Literal};

mod default;
mod extract;
mod into_output;

/// User provided information about a command
pub struct CommandInfo<'a> {
    /// The name of the program
    name: Option<Literal>,

    /// The description of the program
    description: Option<Vec<Expression<'a>>>,

    /// If a version flag should be included, and what content it should have
    version: Option<Option<Expression<'a>>>,

    /// Should a help flag be included
    help: bool,

    /// The user provided header for usage
    usage_header: Option<Expression<'a>>,

    /// The user provided header for positionals
    positional_header: Option<Expression<'a>>,

    /// The user provided header for flags
    flag_header: Option<Expression<'a>>,
}
