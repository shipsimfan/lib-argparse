use super::{FlagLongName, FlagShortName, FlagUnwrap, InProgress, NewInProgress, StructOutput};
use crate::command::FlagInfo;
use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

impl<'a> StructOutput<'a> {
    /// Creates a new [`StructOutput`]
    pub fn new(
        name: Cow<'a, Identifier>,
        infos: Vec<FlagInfo<'a>>,
        in_progress: InProgress<'a>,
        new_in_progress: NewInProgress,
        long_names: Vec<FlagLongName>,
        short_names: Vec<FlagShortName>,
        unwraps: Vec<FlagUnwrap<'a>>,
    ) -> Self {
        let module_name = Identifier::new(&format!("__flag_group_{}", name));

        StructOutput {
            name,
            module_name,
            infos,
            in_progress,
            new_in_progress,
            short_names,
            long_names,
            unwraps,
        }
    }
}
