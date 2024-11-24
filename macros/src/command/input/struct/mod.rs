use positional::Positional;
use proc_macro_util::tokens::Identifier;

mod positional;

mod extract;
mod into_output;

/// The details extracted from a struct
pub struct StructInput<'a> {
    /// The name of the struct
    name: Identifier,

    /// The positionals in this structs
    positionals: Vec<Positional<'a>>,
}
