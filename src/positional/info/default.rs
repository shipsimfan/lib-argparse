use crate::{Positional, PositionalInfo};

impl<T: Positional> Default for PositionalInfo<T> {
    fn default() -> Self {
        PositionalInfo {
            value: "",
            min_count: 0,
            max_count: 0,
            default: None,
        }
    }
}
