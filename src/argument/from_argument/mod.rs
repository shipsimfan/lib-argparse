use crate::{Argument, Error};

/// Convert an argument to a type
pub trait FromArgument<'a>: Sized {
    /// Attempt to convert `argument` into this type
    fn from_argument(argument: Argument<'a>) -> Result<Self, Error>;
}
