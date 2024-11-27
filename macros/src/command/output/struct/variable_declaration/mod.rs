use proc_macro_util::tokens::Identifier;

mod new;
mod to_tokens;

/// A declaration of a variable to be used in parsing
pub struct VariableDeclaration {
    /// The name of the variable
    variable_name: Identifier,
}
