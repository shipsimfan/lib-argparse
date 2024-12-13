use super::Flag;
use crate::command::output::{
    DefaultValue, Description, FlagHelpOutput, FlagHelpUsageOutput, FlagInfo, FlagLongName,
    FlagShortName, FlagUnwrap, VariableDeclaration,
};

impl<'a> Flag<'a> {
    pub fn into_output(
        self,
        description_offset: usize,
        short_names: bool,
    ) -> (
        FlagInfo<'a>,
        VariableDeclaration,
        FlagLongName,
        Option<FlagShortName>,
        FlagUnwrap,
        FlagHelpUsageOutput,
        FlagHelpOutput,
    ) {
        (
            FlagInfo::new(
                self.info_name.clone(),
                self.r#type,
                self.info_long_name.into(),
                self.info_short_name.into(),
                self.value.into(),
                self.min_count,
                self.max_count,
                self.default.map(DefaultValue::new).into(),
                self.description.map(Description::new).into(),
            ),
            VariableDeclaration::new(self.variable_name.clone()),
            FlagLongName::new(
                self.long_name,
                self.variable_name.clone(),
                self.info_name.clone(),
            ),
            self.short_name.map(|short_name| {
                FlagShortName::new(
                    short_name,
                    self.variable_name.clone(),
                    self.info_name.clone(),
                )
            }),
            FlagUnwrap::new(self.variable_name, self.info_name.clone()),
            FlagHelpUsageOutput::new(self.info_name.clone()),
            FlagHelpOutput::new(self.info_name, description_offset, short_names),
        )
    }
}
