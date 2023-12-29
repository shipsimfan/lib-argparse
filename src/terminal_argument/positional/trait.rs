use crate::Result;
use std::{ffi::OsString, num::NonZeroUsize};

/// A single positional argument
pub trait PositionalArgument<'a, Options: 'a> {
    /// Gets the name of the positional
    fn name(&self) -> &dyn std::fmt::Display;

    /// Gets the description of the positional
    fn description(&self) -> &[&dyn std::fmt::Display];

    /// Gets the hint to be displayed in the usage
    fn hint(&self) -> Option<&dyn std::fmt::Display>;

    /// Max number of arguments this positional expects
    ///
    /// Ignored when used as the collecting positional
    fn count(&self) -> NonZeroUsize;

    /// The action called when this positional is reached with an argument
    fn action(&self, index: usize, options: &mut Options, parameter: OsString) -> Result<'a, ()>;

    /// Called for all positionals at the end of parsing
    fn finalize(&self, count: usize) -> Result<'a, ()>;
}
