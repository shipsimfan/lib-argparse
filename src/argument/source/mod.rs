use crate::Argument;

mod args;
mod args_os;

pub use args::ArgsSource;
pub use args_os::ArgsOsSource;

/// A source of arguments
pub trait ArgumentSource<'a> {
    /// Get the next argument from the source
    fn next(&mut self) -> Option<Argument<'a>>;

    /// Are there no more arguments?
    fn empty(&self) -> bool;

    /// Return the name of the program, if this source provides one
    fn program_name(&self) -> Option<&Argument<'a>>;
}
