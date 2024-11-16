use crate::{Argument, Error, Result};

mod info;
mod result;

pub use info::PositionalInfo;
pub use result::PositionalResult;

/// A type which can be a positional argument
pub trait Positional: Sized {
    /// Parse `argument` into this value
    fn parse(
        this: &mut Option<Self>,
        argument: Argument,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult;

    /// Unwrap this positional argument
    fn unwrap(this: Option<Self>, info: &PositionalInfo<Self>) -> Result<Self> {
        match (this, info.default) {
            (Some(value), _) => Ok(value),
            (None, Some(default)) => Ok(default()),
            (None, None) => Err(Error::missing_positional_value(info.value)),
        }
    }
}
