use crate::{Positional, PositionalInfo};

impl<T: Positional> PositionalInfo<T> {
    /// Copies this info into a new struct without copying the default value
    pub fn drop_default<T2: Positional>(&self) -> PositionalInfo<T2> {
        PositionalInfo {
            value: self.value,
            min_count: self.min_count,
            max_count: self.max_count,
            default: None,
        }
    }
}
