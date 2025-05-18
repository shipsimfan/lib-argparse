use flag::Flag;
use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

mod flag;

mod extract;
mod into_output;

/// The details extracted from a struct
pub struct StructInput<'a> {
    /// The name of the struct
    name: Cow<'a, Identifier>,

    /// The flags in the struct
    flags: Vec<Flag<'a>>,
}
