use crate::Error;
use std::{ffi::OsString, ops::Deref};

mod collect;
mod parser;
mod simple;

pub use collect::{CollectOsPositionalParser, CollectPositionalParser};
pub use parser::PositionalParser;
pub use simple::SimplePositionalParser;

/// An ordered list of positional arguments
pub struct Positionals<T, E> {
    /// List of positional parsers
    positionals: Vec<Box<dyn PositionalParser<Options = T, Error = E>>>,

    /// Current positional for parsing
    current_positional: usize,
}

impl<T, E> Positionals<T, E> {
    /// Creates a new empty `Positionals`
    pub fn new() -> Self {
        Positionals {
            positionals: Vec::new(),
            current_positional: 0,
        }
    }

    /// Pushes `positional` to the end of the list
    pub fn push(&mut self, positional: impl PositionalParser<Options = T, Error = E> + 'static) {
        self.positionals.push(Box::new(positional))
    }

    pub(super) fn parse(&mut self, options: &mut T, argument: OsString) -> Result<(), Error<E>> {
        if self.current_positional >= self.positionals.len() {
            return Err(Error::UnexpectedArgument(argument));
        }

        if !self.positionals[self.current_positional].parse(options, argument)? {
            self.current_positional += 1;
        }

        Ok(())
    }

    pub(super) fn finalize(&mut self, options: &mut T) -> Result<(), Error<E>> {
        self.current_positional = 0;

        for positional in &mut self.positionals {
            positional.finalize(options)?;
        }

        Ok(())
    }
}

impl<T, E> Deref for Positionals<T, E> {
    type Target = [Box<dyn PositionalParser<Options = T, Error = E>>];

    fn deref(&self) -> &Self::Target {
        &self.positionals
    }
}
