use super::Positional;
use crate::command::output::{DefaultValue, OptionalOutput, PositionalInfo};

impl<'a> Positional<'a> {
    pub fn into_output(self) -> PositionalInfo<'a> {
        PositionalInfo::new(
            self.info_name,
            self.r#type,
            self.value,
            self.min_count,
            self.max_count,
            match self.default {
                Some(default) => OptionalOutput::Some(DefaultValue::new(default)),
                None => OptionalOutput::None,
            },
        )
    }
}
