use proc_macro_util::tokens::{Identifier, Literal};

mod new;
mod to_tokens;

/// Generates the match arm for a flag's long name
pub struct FlagLongName {
    /// The long name of the flag
    long_name: Literal,

    /// The index of the flag in the in-progress tuple
    index: usize,

    /// The name of the info describing this flag
    info_name: Identifier,
}
