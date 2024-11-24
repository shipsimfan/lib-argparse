use super::StructOutput;
use proc_macro_util::tokens::Identifier;

impl StructOutput {
    /// Creates a new [`StructOutput`]
    pub fn new(name: Identifier) -> Self {
        StructOutput { name }
    }
}
