use super::Flag;
use crate::{
    command::{DefaultValue, Description, FlagInfo},
    flag_group::output::{FlagLongName, FlagShortName, FlagUnwrap},
};
use proc_macro_util::ast::Type;

impl<'a> Flag<'a> {
    /// Converts this flag into the appropriate output types
    pub fn into_output(
        self,
        index: usize,
    ) -> (
        FlagInfo<'a>,
        Type<'a>,
        FlagLongName,
        Option<FlagShortName>,
        FlagUnwrap<'a>,
    ) {
        (
            FlagInfo::new(
                self.info_name.clone(),
                self.r#type.clone(),
                self.info_long_name.into(),
                self.info_short_name.into(),
                self.value.into(),
                self.min.into(),
                self.max.into(),
                self.default.map(DefaultValue::new).into(),
                self.description.map(Description::new).into(),
            ),
            self.r#type,
            FlagLongName::new(self.long_name, index, self.info_name.clone()),
            self.short_name
                .map(|short_name| FlagShortName::new(short_name, index, self.info_name.clone())),
            FlagUnwrap::new(self.variable_name, index, self.info_name),
        )
    }
}
