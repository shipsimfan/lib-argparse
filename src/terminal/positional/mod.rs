use crate::Error;
use std::{borrow::Cow, ffi::OsString, ops::Deref};

mod collect;
mod parser;
mod simple;

pub use collect::{CollectOsPositionalParser, CollectPositionalParser};
pub use parser::PositionalParser;
pub use simple::SimplePositionalParser;

/// An ordered list of positional arguments
pub struct Positionals<T, E> {
    /// List of positional parsers
    ///
    /// The first member of the tuple is the help description.
    /// The seciond member is the parser
    positionals: Vec<(
        Cow<'static, str>,
        Box<dyn PositionalParser<Options = T, Error = E>>,
    )>,

    /// Current positional for parsing
    current_positional: usize,
}

struct UsageDisplayer<'a, T, E>(&'a dyn PositionalParser<Options = T, Error = E>);

impl<T, E> Positionals<T, E> {
    /// Creates a new empty [`Positionals`]
    pub fn new() -> Self {
        Positionals {
            positionals: Vec::new(),
            current_positional: 0,
        }
    }

    /// Pushes a positional parser to the end of the list
    ///
    ///  - `positional` is the positional parser to be pushed onto the list
    ///  - `help_description` is the description of this positional displayed during help
    pub fn push<S: Into<Cow<'static, str>>>(
        &mut self,
        positional: impl PositionalParser<Options = T, Error = E> + 'static,
        help_description: S,
    ) {
        self.positionals
            .push((help_description.into(), Box::new(positional)))
    }

    pub(super) fn generate_usage(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (_, positional) in &self.positionals {
            positional.generate_usage(f)?;
        }

        Ok(())
    }

    /// Generates the help display
    ///
    ///  - `f` is the output
    pub(super) fn generate_help(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "ARGUMENTS:")?;

        let mut longest_name = 0;
        let mut lengths = Vec::with_capacity(self.positionals.len());
        for (_, positional) in &self.positionals {
            let length = UsageDisplayer(positional.as_ref()).to_string().len();
            if length > longest_name {
                longest_name = length;
            }

            lengths.push(length);
        }

        let padding = longest_name + 2;

        for i in 0..self.positionals.len() {
            write!(f, "  ")?;
            self.positionals[i].1.generate_usage(f)?;

            for _ in 0..padding - lengths[i] {
                write!(f, " ")?;
            }

            writeln!(f, "{}", self.positionals[i].0)?;
        }

        Ok(())
    }

    /// Parses a positional argument
    ///
    ///  - `options` is the developer provided options to be updated
    ///  - `argument` is the argument is the positional argument from the stream
    pub(super) fn parse(&mut self, options: &mut T, argument: OsString) -> Result<(), Error<E>> {
        if self.current_positional >= self.positionals.len() {
            return Err(Error::UnexpectedArgument(argument));
        }

        if !self.positionals[self.current_positional]
            .1
            .parse(options, argument)?
        {
            self.current_positional += 1;
        }

        Ok(())
    }

    /// Finalizes all the positional arguments in the list
    ///
    ///  - `options` is the developer provided options to be updated
    pub(super) fn finalize(&mut self, options: &mut T) -> Result<(), Error<E>> {
        self.current_positional = 0;

        for (_, positional) in &mut self.positionals {
            positional.finalize(options)?;
        }

        Ok(())
    }
}

impl<T, E> Deref for Positionals<T, E> {
    type Target = [(
        Cow<'static, str>,
        Box<dyn PositionalParser<Options = T, Error = E>>,
    )];

    fn deref(&self) -> &Self::Target {
        &self.positionals
    }
}

impl<'a, T, E> std::fmt::Display for UsageDisplayer<'a, T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.generate_usage(f)
    }
}
