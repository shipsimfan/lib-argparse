use super::{
    FlagGroupDeclaration, FlagGroupInProgress, FlagGroupLongName, FlagGroupShortName,
    FlagGroupUnwrap, FlagLongName, FlagShortName, FlagUnwrap, InProgress, NewInProgress,
    StructOutput,
};
use crate::command::{
    FlagGroupHelpOutput, FlagGroupHelpUsageOutput, FlagHelpOutput, FlagHelpUsageOutput, FlagInfo,
};
use proc_macro_util::{
    ast::{GenericArgs, GenericParams},
    tokens::Identifier,
};
use std::borrow::Cow;

impl<'a> StructOutput<'a> {
    /// Creates a new [`StructOutput`]
    pub fn new(
        name: Cow<'a, Identifier>,
        generic_params: Option<GenericParams<'a>>,
        generic_args: Option<GenericArgs<'a>>,
        infos: Vec<FlagInfo<'a>>,
        in_progress: InProgress<'a>,
        new_in_progress: NewInProgress,
        long_names: Vec<FlagLongName>,
        short_names: Vec<FlagShortName>,
        unwraps: Vec<FlagUnwrap<'a>>,
        usages: Vec<FlagHelpUsageOutput>,
        helps: Vec<FlagHelpOutput>,
        flag_group_in_progress: Vec<FlagGroupInProgress<'a>>,
        flag_group_declarations: Vec<FlagGroupDeclaration<'a>>,
        flag_group_long_names: Vec<FlagGroupLongName<'a>>,
        flag_group_short_names: Vec<FlagGroupShortName<'a>>,
        flag_group_unwraps: Vec<FlagGroupUnwrap<'a>>,
        flag_group_usages: Vec<FlagGroupHelpUsageOutput<'a>>,
        flag_group_helps: Vec<FlagGroupHelpOutput<'a>>,
    ) -> Self {
        let module_name = Identifier::new(&format!("__flag_group_{}", name));

        StructOutput {
            name,
            generic_params,
            generic_args,
            module_name,
            infos,
            in_progress,
            new_in_progress,
            short_names,
            long_names,
            unwraps,
            helps,
            usages,
            flag_group_in_progress,
            flag_group_declarations,
            flag_group_long_names,
            flag_group_short_names,
            flag_group_unwraps,
            flag_group_usages,
            flag_group_helps,
        }
    }
}
