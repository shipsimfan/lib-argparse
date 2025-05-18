use crate::{Positional, PositionalInfo};

impl<T: Positional> PositionalInfo<T> {
    /// Copies this info into a new struct without copying the default value
    pub fn drop_default<T2: Positional>(&self) -> PositionalInfo<T2> {
        PositionalInfo {
            value: self.value,
            min: self.min,
            max: self.max,
            default: None,
            description: self.description,
        }
    }
}
