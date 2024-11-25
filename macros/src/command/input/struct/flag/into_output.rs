use super::Flag;
use crate::command::output::{DefaultValue, FlagInfo};

impl<'a> Flag<'a> {
    pub fn into_output(self) -> FlagInfo<'a> {
        FlagInfo::new(
            self.info_name,
            self.r#type,
            self.long_name.into(),
            self.short_name.into(),
            self.value.into(),
            self.min_count,
            self.max_count,
            self.default.map(DefaultValue::new).into(),
        )
    }
}
