use crate::Argument;

mod env;

/// A source of arguments
pub trait ArgumentSource<'a> {
    /// Get the next argument from the source
    fn next(&mut self) -> Option<Argument<'a>>;

    /// Are there no more arguments?
    fn empty(&self) -> bool;
}
