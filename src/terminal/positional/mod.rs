use std::ops::Deref;

mod parser;

pub use parser::PositionalParser;

pub struct Positionals<T, E>(Vec<Box<dyn PositionalParser<Options = T, Error = E>>>);

impl<T, E> Positionals<T, E> {
    pub fn new() -> Self {
        Positionals(Vec::new())
    }

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
