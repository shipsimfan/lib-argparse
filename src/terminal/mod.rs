use crate::{Error, Parser};
use std::{borrow::Cow, ffi::OsString};

mod command;
mod positional;

pub use command::Command;
pub use positional::{
    CollectOsPositionalParser, CollectPositionalParser, PositionalParser, Positionals,
    SimplePositionalParser,
};

pub enum TerminalArgument<T, E: 'static> {
    None,
    Command(Command<T, E>),
    Positionals(Positionals<T, E>),
}

impl<T, E> TerminalArgument<T, E> {
    /// Sets this command to be required
    ///
    /// Can only be applied to [`TerminalArgument::Command`]
    ///
    ///  - `missing_error_message` is the error message displayed if this command is missing
    pub fn set_required<S: Into<Cow<'static, str>>>(self, missing_error_message: S) -> Self {
        match self {
            TerminalArgument::None => {
                panic!("Cannot set a TerminalArgument::None to required")
            }
            TerminalArgument::Command(command) => {
                TerminalArgument::Command(command.set_required(missing_error_message))
            }
            TerminalArgument::Positionals(_) => {
                panic!("Cannot set a TerminalArgument::Positionals to required")
            }
        }
    }

    /// Sets this command to be not required
    pub fn set_not_required(self) -> Self {
        match self {
            TerminalArgument::Command(command) => {
                TerminalArgument::Command(command.set_not_required())
            }
            _ => self,
        }
    }

    /// Generates the help usage for this terminal argument
    ///
    ///  - `f` is the output
    pub(crate) fn generate_usage(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TerminalArgument::None => Ok(()),
            TerminalArgument::Command(command) => command.generate_usage(f),
            TerminalArgument::Positionals(positionals) => positionals.generate_usage(f),
        }
    }

    /// Generates the help display
    ///
    ///  - `f` is the output
    pub(crate) fn generate_help(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TerminalArgument::None => Ok(()),
            TerminalArgument::Command(command) => command.generate_help(f),
            TerminalArgument::Positionals(positionals) => positionals.generate_help(f),
        }
    }

    /// Parses a terminal argument
    ///
    /// Returns if a command was parsed
    ///  - `true` if a command was parsed
    ///  - `false` if a command was not parsed
    ///
    ///  - `options` is the developer provided options to be updated
    ///  - `argument` is the argument to be parsed
    pub(crate) fn parse(&mut self, options: &mut T, argument: OsString) -> Result<bool, Error<E>> {
        match self {
            TerminalArgument::None => Err(Error::UnexpectedArgument(argument)),
            TerminalArgument::Command(command) => command.parse(options, argument).map(|_| true),
            TerminalArgument::Positionals(positionals) => {
                positionals.parse(options, argument).map(|_| false)
            }
        }
    }

    /// Finalizes a terminal argument
    ///
    ///  - `options` is the developer provided options to be updated
    pub(crate) fn finalize(
        &mut self,
        options: &mut T,
    ) -> Result<Option<(&mut Parser<T, E>, String)>, Error<E>> {
        match self {
            TerminalArgument::None => Ok(None),
            TerminalArgument::Command(command) => command.finalize(),
            TerminalArgument::Positionals(positionals) => {
                positionals.finalize(options).map(|_| None)
            }
        }
    }
}
