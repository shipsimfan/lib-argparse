use std::ops::Deref;

mod collect;
mod parser;
mod simple;

pub use collect::{CollectOsPositionalParser, CollectPositionalParser};
pub use parser::PositionalParser;
pub use simple::SimplePositionalParser;

/// An ordered list of positional arguments
pub struct Positionals<T, E>(Vec<Box<dyn PositionalParser<Options = T, Error = E>>>);

impl<T, E> Positionals<T, E> {
    /// Creates a new empty `Positionals`
    pub fn new() -> Self {
        Positionals(Vec::new())
    }

    /// Pushes `positional` to the end of the list
    pub fn push(&mut self, positional: impl PositionalParser<Options = T, Error = E> + 'static) {
        self.0.push(Box::new(positional))
    }
}

impl<T, E> Deref for Positionals<T, E> {
    type Target = [Box<dyn PositionalParser<Options = T, Error = E>>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
