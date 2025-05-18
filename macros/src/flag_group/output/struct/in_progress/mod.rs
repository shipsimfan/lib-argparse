use proc_macro_util::ast::Type;

mod new;
mod to_tokens;

/// Produces the tokens which define the in-progress types
pub struct InProgress<'a> {
    /// The types in the tuple
    types: Vec<Type<'a>>,
}
