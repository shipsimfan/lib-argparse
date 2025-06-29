use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

mod new;
mod to_tokens;

/// A declaration of a variable to be used in parsing
pub struct VariableDeclaration<'a> {
    /// The name of the variable
    variable_name: Cow<'a, Identifier>,
}
