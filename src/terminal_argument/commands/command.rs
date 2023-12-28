use crate::{Parser, Result};
use std::marker::PhantomData;

/// A command
pub struct Command<'a, Options: 'a> {
    name: &'a str,
    description: Option<&'a [&'a dyn std::fmt::Display]>,
    action: &'a dyn Fn(&mut Options) -> Result<'a, ()>,
    parser: Option<&'a Parser<'a, Options>>,
    phantom: PhantomData<Options>,
}

impl<'a, Options: 'a> Command<'a, Options> {
    /// Creates a new [`Command`]
    pub const fn new(name: &'a str, action: &'a dyn Fn(&mut Options) -> Result<'a, ()>) -> Self {
        Command {
            name,
            description: None,
            action,
            parser: None,
            phantom: PhantomData,
        }
    }

    /// Sets the name of a command
    pub const fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }

    /// Sets the description of a command
    pub const fn description(mut self, description: &'a [&'a dyn std::fmt::Display]) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the action for a command
    pub const fn action(mut self, action: &'a dyn Fn(&mut Options) -> Result<'a, ()>) -> Self {
        self.action = action;
        self
    }

    /// Sets the parser for a command
    pub const fn parser(mut self, parser: &'a Parser<'a, Options>) -> Self {
        self.parser = Some(parser);
        self
    }

    /// Gets the name of this command
    pub(super) fn get_name(&self) -> &str {
        self.name
    }

    /// Gets the description of this command
    pub(super) fn get_description(&self) -> Option<&[&dyn std::fmt::Display]> {
        self.description
    }

    /// Runs this command's action
    pub(super) fn do_action(
        &self,
        options: &mut Options,
    ) -> Result<'a, Option<&Parser<'a, Options>>> {
        (self.action)(options).map(|_| self.parser)
    }
}
