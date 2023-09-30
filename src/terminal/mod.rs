use crate::{Error, Parser};
use std::ffi::OsString;

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
    /// Parses a terminal argument
    ///
    /// Returned `usize` is the index of the selected command
    pub(crate) fn parse(&mut self, options: &mut T, argument: OsString) -> Result<bool, Error<E>> {
        match self {
            TerminalArgument::None => Err(Error::UnexpectedArgument(argument)),
            TerminalArgument::Command(command) => command.parse(argument).map(|_| true),
            TerminalArgument::Positionals(positionals) => {
                positionals.parse(options, argument).map(|_| false)
            }
        }
    }

    /// Finalizes a terminal argument
    ///
    /// `command_index` si the index returned from `parse()`
    pub(crate) fn finalize(
        &mut self,
        options: &mut T,
    ) -> Result<Option<&mut Parser<T, E>>, Error<E>> {
        match self {
            TerminalArgument::None => Ok(None),
            TerminalArgument::Command(command) => command.finalize(),
            TerminalArgument::Positionals(positionals) => {
                positionals.finalize(options).map(|_| None)
            }
        }
    }
}
