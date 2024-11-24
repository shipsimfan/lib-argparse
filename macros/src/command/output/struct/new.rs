use super::{positional::PositionalInfo, StructOutput};
use proc_macro_util::tokens::Identifier;

impl<'a> StructOutput<'a> {
    /// Creates a new [`StructOutput`]
    pub fn new(name: Identifier, positional_info: Vec<PositionalInfo<'a>>) -> Self {
        StructOutput {
            name,
            positional_info,
        }
    }
}
