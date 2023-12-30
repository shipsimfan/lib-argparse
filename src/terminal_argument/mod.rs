use crate::{Parser, Result};
use std::ffi::OsString;

mod commands;
mod positional;

pub use commands::{Command, Commands};
pub use positional::{
    ParsingPositionalArgument, PositionalArgument, PositionalTerminalArgument,
    SimplePositionalArgument,
};

/// An argument that consumes all non-flag arguments
pub trait TerminalArgument<'a, Options: 'a> {
    /// The action called when an non-flag argument is encountered
    ///
    /// If the action returns a [`Parser`], then the remaining arguments will be parsed with that
    /// parser instead of the current one
    fn action(
        &self,
        options: &mut Options,
        index: usize,
        parameter: OsString,
    ) -> Result<'a, Option<&Parser<'a, Options>>>;

    /// Called for only the current parser's terminal argument when the end of arguments are
    /// reached
    fn finalize(&self, count: usize) -> Result<'a, ()>;

    /// Gets the hint for displaying in the help
    fn hint(&self) -> &dyn std::fmt::Display;

    /// Writes the help for this terminal argument to `f`
    fn help(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result;
}
