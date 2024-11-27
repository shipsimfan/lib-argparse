use proc_macro_util::tokens::Identifier;

mod new;
mod to_tokens;

/// Unwraps a flag variable
pub struct FlagUnwrap {
    /// The name of the variable to unwrap
    variable_name: Identifier,

    /// The name of the information describing the flag
    info_name: Identifier,
}
