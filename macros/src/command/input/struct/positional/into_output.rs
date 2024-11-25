use super::Positional;
use crate::command::output::{DefaultValue, PositionalInfo};

impl<'a> Positional<'a> {
    pub fn into_output(self) -> PositionalInfo<'a> {
        PositionalInfo::new(
            self.info_name,
            self.r#type,
            self.value,
            self.min_count,
            self.max_count,
            self.default.map(DefaultValue::new).into(),
        )
    }
}
