use crate::{Argument, ArgumentSource, DefaultDisplay, Error, Result};

mod info;
mod result;

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
mod time;

pub use info::PositionalInfo;
pub use result::PositionalResult;

/// A type which can be a positional argument
pub trait Positional: Sized + DefaultDisplay {
    /// Parse `argument` into this value
    fn parse<'a>(
        this: &mut Option<Self>,
        argument: Argument<'a>,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult<'a>;

    /// Unwrap this positional argument
    fn unwrap(this: Option<Self>, info: &PositionalInfo<Self>) -> Result<Self> {
        match (this, info.default) {
            (Some(value), _) => Ok(value),
            (None, Some(default)) => Ok(default()),
            (None, None) => Err(Error::missing_positional_value(info.value)),
        }
    }

    /// Continue parsing as a sub-command, returning true if the parse should return [`Some`]
    #[allow(unused_variables)]
    fn sub(
        this: &mut Option<Self>,
        command: Argument,
        source: &mut dyn ArgumentSource,
        command_list: String,
    ) -> Result<bool> {
        unimplemented!()
    }

    /// Is this positional required?
    fn is_required(info: &PositionalInfo<Self>) -> bool {
        info.default.is_none()
    }

    /// Can this positional take multiple values
    #[allow(unused_variables)]
    fn multiple(info: &PositionalInfo<Self>) -> bool {
        false
    }
}
