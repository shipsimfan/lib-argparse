use proc_macro_util::tokens::Identifier;

mod new;
mod to_tokens;

/// The output code for a struct
pub struct StructOutput {
    /// The name of the struct
    pub(super) name: Identifier,
}
