use super::FlagLongName;
use proc_macro_util::tokens::{Identifier, Literal};

impl FlagLongName {
    /// Creates a new [`FlagLongName`]
    pub fn new(long_name: Literal, variable_name: Identifier, info_name: Identifier) -> Self {
        FlagLongName {
            long_name,
            variable_name,
            info_name,
        }
    }
}
