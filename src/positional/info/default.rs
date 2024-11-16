use crate::PositionalInfo;

impl<T> Default for PositionalInfo<T> {
    fn default() -> Self {
        PositionalInfo {
            value: "",
            min_count: 0,
            max_count: 0,
            default: None,
        }
    }
}
