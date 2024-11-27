use super::FlagUnwrap;
use proc_macro_util::tokens::Identifier;

impl FlagUnwrap {
    /// Creates a new [`FlagUnwrap`]
    pub fn new(variable_name: Identifier, info_name: Identifier) -> Self {
        FlagUnwrap {
            variable_name,
            info_name,
        }
    }
}
