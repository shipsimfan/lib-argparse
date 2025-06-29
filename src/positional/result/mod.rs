use crate::{Argument, Error};

mod from_residual;
mod r#try;

/// A result of parsing a positional
#[derive(Debug)]
pub enum PositionalResult<'a> {
    /// Continue parsing this positional by passing the next argument to it
    Continue,

    /// This positional is done parsing arguments and the next positional can be parsed
    Next,

    /// This positional is a sub-command, continue parsing using it
    Sub(Argument<'a>),

    /// An error occurred while parsing
    Error(Error),
}
