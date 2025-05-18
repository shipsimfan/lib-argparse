use proc_macro_util::tokens::{Identifier, Literal};

mod new;
mod to_tokens;

/// Generates the match arm for a flag's short name
pub struct FlagShortName {
    /// The short name of the flag
    short_name: Literal,

    /// The index of the flag in the in-progress tuple
    index: usize,

    /// The name of the info describing this flag
    info_name: Identifier,
}
