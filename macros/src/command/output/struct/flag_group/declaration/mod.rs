use proc_macro_util::{ast::Type, tokens::Identifier};
use std::borrow::Cow;

mod new;
mod to_tokens;

/// A declaration of a variable for a flag group to be used in parsing
pub struct FlagGroupDeclaration<'a> {
    /// The name of the variable for the flag group
    variable_name: Cow<'a, Identifier>,

    /// The type of the flag group
    r#type: Type<'a>,
}
