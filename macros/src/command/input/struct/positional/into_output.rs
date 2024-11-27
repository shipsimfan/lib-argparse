use super::Positional;
use crate::command::output::{DefaultValue, PositionalInfo, PositionalUnwrap, VariableDeclaration};

impl<'a> Positional<'a> {
    pub fn into_output(self) -> (PositionalInfo<'a>, VariableDeclaration, PositionalUnwrap) {
        (
            PositionalInfo::new(
                self.info_name.clone(),
                self.r#type,
                self.value,
                self.min_count,
                self.max_count,
                self.default.map(DefaultValue::new).into(),
            ),
            VariableDeclaration::new(self.variable_name.clone()),
            PositionalUnwrap::new(self.variable_name, self.info_name),
        )
    }
}
