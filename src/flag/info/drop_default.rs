use crate::{Flag, FlagInfo};

impl<T: Flag> FlagInfo<T> {
    /// Copies this info into a new struct without copying the default value
    pub fn drop_default<T2: Flag>(&self) -> FlagInfo<T2> {
        FlagInfo {
            long_name: self.long_name,
            short_name: self.short_name,
            value: self.value,
            min_count: self.min_count,
            max_count: self.max_count,
            default: None,
            description: self.description,
        }
    }
}
