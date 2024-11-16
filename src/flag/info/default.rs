use crate::FlagInfo;

impl<T> Default for FlagInfo<T> {
    fn default() -> Self {
        FlagInfo {
            argument: "",
            value: None,
            min_count: 0,
            max_count: 0,
            default: None,
        }
    }
}
