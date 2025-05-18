use crate::{ArgumentSource, DefaultDisplay, Error, Result};

mod info;

mod array;
mod bool;
mod borrow;
mod r#box;
mod cell;
mod char;
mod collections;
mod net;
mod number;
mod option;
mod path;
mod rc;
mod string;
mod sync;
mod tuple;

pub use info::FlagInfo;

pub(crate) const DEFUALT_FLAG_VALUE: &str = "VALUE";

/// A type which can parsed from a flag
pub trait Flag: Sized + DefaultDisplay {
    /// Parse arguments from `source` to get the value
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()>;

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

    /// Is this flag required?
    fn is_required(info: &FlagInfo<Self>) -> bool {
        info.default.is_none()
    }

    /// Does this flag take a value
    #[allow(unused_variables)]
    fn takes_value(info: &FlagInfo<Self>) -> bool {
        true
    }
}
