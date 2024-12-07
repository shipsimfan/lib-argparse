use super::{
    FlagInfo, FlagLongName, FlagShortName, FlagUnwrap, PositionalInfo, PositionalMatch,
    PositionalSubCommand, PositionalUnwrap, StructOutput, VariableDeclaration,
};
use crate::command::output::VersionOutput;
use proc_macro_util::tokens::Identifier;

impl<'a> StructOutput<'a> {
    /// Creates a new [`StructOutput`]
    pub fn new(
        name: Identifier,
        positional_info: Vec<PositionalInfo<'a>>,
        positional_declarations: Vec<VariableDeclaration>,
        positional_matches: Vec<PositionalMatch>,
        positional_sub_commands: Vec<PositionalSubCommand>,
        positional_unwraps: Vec<PositionalUnwrap>,
        flag_info: Vec<FlagInfo<'a>>,
        flag_declarations: Vec<VariableDeclaration>,
        flag_long_names: Vec<FlagLongName>,
        flag_short_names: Vec<FlagShortName>,
        flag_unwraps: Vec<FlagUnwrap>,
        version: Option<VersionOutput<'a>>,
    ) -> Self {
        StructOutput {
            name,
            positional_info,
            positional_declarations,
            positional_matches,
            positional_sub_commands,
            positional_unwraps,
            flag_info,
            flag_declarations,
            flag_long_names,
            flag_short_names,
            flag_unwraps,
            version,
        }
    }
}
