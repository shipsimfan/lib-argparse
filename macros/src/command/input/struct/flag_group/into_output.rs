use super::FlagGroup;
use crate::command::output::{
    FlagGroupDeclaration, FlagGroupLongName, FlagGroupShortName, FlagGroupUnwrap,
};

impl<'a> FlagGroup<'a> {
    pub fn into_output(
        self,
    ) -> (
        FlagGroupDeclaration<'a>,
        FlagGroupLongName<'a>,
        FlagGroupShortName<'a>,
        FlagGroupUnwrap<'a>,
    ) {
        (
            FlagGroupDeclaration::new(self.variable_name.clone(), self.r#type.clone()),
            FlagGroupLongName::new(self.variable_name.clone(), self.r#type.clone()),
            FlagGroupShortName::new(self.variable_name.clone(), self.r#type),
            FlagGroupUnwrap::new(self.variable_name),
        )
    }
}
