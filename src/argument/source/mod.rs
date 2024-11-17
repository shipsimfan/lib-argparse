use crate::Argument;

mod env;

/// A source of arguments
pub trait ArgumentSource<'a> {
    /// Get the next argument from the source
    fn next(&mut self) -> Option<Argument<'a>>;

    /// Are there no more arguments?
    fn empty(&self) -> bool;

    /// Is the first argument from this source the name of the program?
    fn first_is_program(&self) -> bool;
}
