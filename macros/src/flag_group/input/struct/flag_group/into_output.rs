use super::FlagGroup;
use crate::{
    command::{FlagGroupHelpOutput, FlagGroupHelpUsageOutput},
    flag_group::output::{
        FlagGroupDeclaration, FlagGroupInProgress, FlagGroupLongName, FlagGroupShortName,
        FlagGroupUnwrap,
    },
};

impl<'a> FlagGroup<'a> {
    pub fn into_output(
        self,
        index: usize,
    ) -> (
        FlagGroupInProgress<'a>,
        FlagGroupDeclaration<'a>,
        FlagGroupLongName<'a>,
        FlagGroupShortName<'a>,
        FlagGroupUnwrap<'a>,
        FlagGroupHelpUsageOutput<'a>,
        FlagGroupHelpOutput<'a>,
    ) {
        (
            FlagGroupInProgress::new(index == 0, self.r#type.clone()),
            FlagGroupDeclaration::new(index == 0, self.r#type.clone()),
            FlagGroupLongName::new(index, self.r#type.clone()),
            FlagGroupShortName::new(index, self.r#type.clone()),
            FlagGroupUnwrap::new(self.variable_name, index),
            FlagGroupHelpUsageOutput::new(self.r#type.clone()),
            FlagGroupHelpOutput::new(self.header_name, self.r#type),
        )
    }
}
