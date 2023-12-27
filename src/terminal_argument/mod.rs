use crate::{Parser, Result};
use std::ffi::OsString;

pub trait TerminalArgument<'a, Options: 'a> {
    /// The action called when an non-flag argument is encountered
    ///
    /// If the action returns a [`Parser`], then the remaining arguments will be parsed with that
    /// parser instead of the current one
    fn action(
        &mut self,
        options: &mut Options,
        parameter: OsString,
    ) -> Result<'a, Option<&Parser<'a, Options>>>;

    /// Called for only the current parser's terminal argument when the end of arguments are
    /// reached
    fn finalize(&mut self) -> Result<'a, ()>;

    /// Gets the hint for displaying in the help
    fn hint(&self) -> &dyn std::fmt::Display;

    /// Writes the help for this terminal argument to `f`
    fn help(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result;
}
