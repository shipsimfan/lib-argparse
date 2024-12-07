use crate::{ArgumentSource, DefaultDisplay, Error, Result};

mod info;

mod array;
mod bool;
mod char;
mod number;
mod string;
mod tuple;

pub use info::FlagInfo;
pub use number::InvalidNumberError;

/// A type which can parsed from a flag
pub trait Flag: Sized + DefaultDisplay {
    /// Parse arguments from `source` to get the value
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self>;

    /// Unwrap this flag
    fn unwrap(this: Option<Self>, info: &FlagInfo<Self>) -> Result<Self> {
        match (this, info.default) {
            (Some(value), _) => Ok(value),
            (None, Some(default)) => Ok(default()),
            (None, None) => Err(Error::missing_argument(
                info.long_name.unwrap_or_else(|| info.short_name.unwrap()),
            )),
        }
    }
}
