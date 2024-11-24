use proc_macro_util::tokens::Identifier;

mod extract;
mod into_output;

/// The details extracted from a struct
pub struct StructInput {
    /// The name of the struct
    name: Identifier,
}
