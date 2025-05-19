use super::FlagGroup;
use crate::command::output::{
    FlagGroupDeclaration, FlagGroupHelpOutput, FlagGroupHelpUsageOutput, FlagGroupLongName,
    FlagGroupShortName, FlagGroupUnwrap,
};

impl<'a> FlagGroup<'a> {
    pub fn into_output(
        self,
    ) -> (
        FlagGroupDeclaration<'a>,
        FlagGroupLongName<'a>,
        FlagGroupShortName<'a>,
        FlagGroupUnwrap<'a>,
        FlagGroupHelpUsageOutput<'a>,
        FlagGroupHelpOutput<'a>,
    ) {
        (
            FlagGroupDeclaration::new(self.variable_name.clone(), self.r#type.clone()),
            FlagGroupLongName::new(self.variable_name.clone(), self.r#type.clone()),
            FlagGroupShortName::new(self.variable_name.clone(), self.r#type.clone()),
            FlagGroupUnwrap::new(self.variable_name),
            FlagGroupHelpUsageOutput::new(self.r#type.clone()),
            FlagGroupHelpOutput::new(self.header_name, self.r#type),
        )
    }
}
