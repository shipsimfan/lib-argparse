use super::Positional;
use crate::command::output::{
    DefaultValue, Description, PositionalHelpOutput, PositionalHelpUsageOutput, PositionalInfo,
    PositionalMatch, PositionalSubCommand, PositionalUnwrap, VariableDeclaration,
};

impl<'a> Positional<'a> {
    pub fn into_output(
        self,
        index: usize,
        description_offset: usize,
    ) -> (
        PositionalInfo<'a>,
        VariableDeclaration<'a>,
        PositionalMatch<'a>,
        PositionalSubCommand<'a>,
        PositionalUnwrap<'a>,
        PositionalHelpUsageOutput,
        PositionalHelpOutput,
    ) {
        (
            PositionalInfo::new(
                self.info_name.clone(),
                self.r#type.clone(),
                self.value,
                self.min,
                self.max,
                self.default.map(DefaultValue::new).into(),
                self.description.map(Description::new).into(),
            ),
            VariableDeclaration::new(self.variable_name.clone()),
            PositionalMatch::new(index, self.variable_name.clone(), self.info_name.clone()),
            PositionalSubCommand::new(index, self.variable_name.clone()),
            PositionalUnwrap::new(self.variable_name, self.info_name.clone()),
            PositionalHelpUsageOutput::new(self.info_name.clone()),
            PositionalHelpOutput::new(self.info_name, description_offset),
        )
    }
}
