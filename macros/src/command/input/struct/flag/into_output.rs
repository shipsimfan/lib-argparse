use super::Flag;
use crate::command::output::{
    DefaultValue, FlagInfo, FlagLongName, FlagShortName, FlagUnwrap, VariableDeclaration,
};

impl<'a> Flag<'a> {
    pub fn into_output(
        self,
    ) -> (
        FlagInfo<'a>,
        VariableDeclaration,
        FlagLongName,
        Option<FlagShortName>,
        FlagUnwrap,
    ) {
        (
            FlagInfo::new(
                self.info_name.clone(),
                self.r#type,
                self.long_name.clone().into(),
                self.short_name.clone().into(),
                self.value.into(),
                self.min_count,
                self.max_count,
                self.default.map(DefaultValue::new).into(),
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
            FlagUnwrap::new(self.variable_name, self.info_name),
        )
    }
}
