use crate::{Argument, ArgumentSource, DefaultDisplay, Error, Result};

mod info;
mod result;

mod collections;
mod path;
mod string;

pub use info::PositionalInfo;
pub use result::PositionalResult;

/// A type which can be a positional argument
pub trait Positional: Sized + DefaultDisplay {
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

    /// Continue parsing as a sub-command
    #[allow(unused_variables)]
    fn sub(this: &mut Option<Self>, parser: &mut dyn ArgumentSource) -> Result<bool> {
        unimplemented!()
    }
}
