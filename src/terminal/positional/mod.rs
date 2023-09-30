use crate::Error;
use std::ops::Deref;

mod collect;
mod parser;
mod simple;

pub use collect::{CollectOsPositionalParser, CollectPositionalParser};
pub use parser::PositionalParser;
pub use simple::SimplePositionalParser;

/// An ordered list of positional arguments
pub struct Positionals<T, E> {
    positionals: Vec<Box<dyn PositionalParser<Options = T, Error = E>>>,
}

impl<T, E> Positionals<T, E> {
    /// Creates a new empty `Positionals`
    pub fn new() -> Self {
        Positionals {
            positionals: Vec::new(),
        }
    }

    /// Pushes `positional` to the end of the list
    pub fn push(&mut self, positional: impl PositionalParser<Options = T, Error = E> + 'static) {
        self.positionals.push(Box::new(positional))
    }

    pub(super) fn parse(&mut self) -> Result<(), Error<E>> {
        todo!("Parse positional")
    }

    pub(super) fn finalize(&mut self) -> Result<(), Error<E>> {
        todo!("Finalize positional");
    }
}

impl<T, E> Deref for Positionals<T, E> {
    type Target = [Box<dyn PositionalParser<Options = T, Error = E>>];

    fn deref(&self) -> &Self::Target {
        &self.positionals
    }
}
