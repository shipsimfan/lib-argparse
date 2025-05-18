use crate::{ArgumentSource, Result};

/// A group of flags that can be used in a larger command or flag group
pub trait FlagGroup: Sized {
    /// The incomplete version of this flag group
    type InProgress: Sized;

    /// Create a new [`Self::InProgress`] to be filled
    fn new_in_progress() -> Self::InProgress;

    /// Check if `flag` is in this group as a long flag, and if so, parse it from `source` and
    /// return `true`
    fn parse_long(
        this: &mut Self::InProgress,
        flag: &str,
        source: &mut dyn ArgumentSource,
    ) -> Result<bool>;

    /// Check if `flag` is in this group as a short flag, and if so, parse it from `source` and
    /// return `true`
    fn parse_short(
        this: &mut Self::InProgress,
        flag: char,
        source: &mut dyn ArgumentSource,
    ) -> Result<bool>;

    /// Unwrap the flags in this group
    fn unwrap(this: Self::InProgress) -> Result<Self>;
}
