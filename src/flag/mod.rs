use crate::{ArgumentSource, Error, Result};

mod info;

mod bool;

pub use info::FlagInfo;

/// A type which can parsed from a flag
pub trait Flag: Sized {
    /// Parse arguments from `source` to get the value
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>) -> Result<Self>;

    /// Unwrap this flag
    fn unwrap(this: Option<Self>, info: &FlagInfo<Self>) -> Result<Self> {
        match (this, info.default) {
            (Some(value), _) => Ok(value),
            (None, Some(default)) => Ok(default()),
            (None, None) => Err(Error::missing_argument(info.argument)),
        }
    }
}
