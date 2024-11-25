use super::{FlagInfo, PositionalInfo, StructOutput};
use proc_macro_util::tokens::Identifier;

impl<'a> StructOutput<'a> {
    /// Creates a new [`StructOutput`]
    pub fn new(
        name: Identifier,
        positional_info: Vec<PositionalInfo<'a>>,
        flag_info: Vec<FlagInfo<'a>>,
    ) -> Self {
        StructOutput {
            name,
            positional_info,
            flag_info,
        }
    }
}
