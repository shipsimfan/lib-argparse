use crate::FlagInfo;

impl<T> Default for FlagInfo<T> {
    fn default() -> Self {
        FlagInfo {
            long_name: None,
            short_name: None,
            value: None,
            min_count: 0,
            max_count: 0,
            default: None,
        }
    }
}