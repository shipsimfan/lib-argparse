use super::{
    FlagGroupDeclaration, FlagGroupLongName, FlagGroupShortName, FlagGroupUnwrap, FlagInfo,
    FlagLongName, FlagShortName, FlagUnwrap, PositionalInfo, PositionalMatch, PositionalSubCommand,
    PositionalUnwrap, StructOutput, VariableDeclaration,
};
use crate::command::output::{HelpOutput, VersionOutput};
use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

impl<'a> StructOutput<'a> {
    /// Creates a new [`StructOutput`]
    pub fn new(
        name: Cow<'a, Identifier>,
        positional_info: Vec<PositionalInfo<'a>>,
        positional_declarations: Vec<VariableDeclaration<'a>>,
        positional_matches: Vec<PositionalMatch<'a>>,
        positional_sub_commands: Vec<PositionalSubCommand<'a>>,
        positional_unwraps: Vec<PositionalUnwrap<'a>>,
        flag_info: Vec<FlagInfo<'a>>,
        flag_declarations: Vec<VariableDeclaration<'a>>,
        flag_long_names: Vec<FlagLongName<'a>>,
        flag_short_names: Vec<FlagShortName<'a>>,
        flag_unwraps: Vec<FlagUnwrap<'a>>,
        flag_group_declarations: Vec<FlagGroupDeclaration<'a>>,
        flag_group_long_names: Vec<FlagGroupLongName<'a>>,
        flag_group_short_names: Vec<FlagGroupShortName<'a>>,
        flag_group_unwraps: Vec<FlagGroupUnwrap<'a>>,
        version: Option<VersionOutput<'a>>,
        help: Option<HelpOutput<'a>>,
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
            flag_group_declarations,
            flag_group_long_names,
            flag_group_short_names,
            flag_group_unwraps,
            version,
            help,
        }
    }
}
