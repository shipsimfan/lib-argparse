use crate::{Positional, PositionalInfo};

impl<T: Positional> Default for PositionalInfo<T> {
    fn default() -> Self {
        PositionalInfo {
            value: "",
            min: None,
            max: None,
            default: None,
            description: None,
        }
    }
}
