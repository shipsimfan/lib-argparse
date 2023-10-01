use crate::{Error, Parser};
use std::{borrow::Cow, ffi::OsString, ops::Deref};

/// An unorded set of commands
pub struct Command<T, E: 'static> {
    /// The list of positionals
    ///
    /// The first member of the tuple is the name of the command.
    /// The second member is the new parser if the command is reached.
    commands: Vec<(Cow<'static, str>, Parser<T, E>)>,

    /// The name of the command
    ///
    /// Placed at the "$" in the unknown command error string "unknown $"
    command_name: Cow<'static, str>,

    /// Determines if this command is required
    ///
    /// The string contained is the error message displayed if this command is missing
    required: Option<Cow<'static, str>>,

    /// The command index found during parsing.
    command_index: Option<usize>,
}

impl<T, E> Command<T, E> {
    /// Creates a new empty `Command` set
    ///
    ///  - `command_name` is the name of the command placed at the "$" in the unknown error string "unknown $"
    pub fn new<S: Into<Cow<'static, str>>>(command_name: S) -> Self {
        Command {
            commands: Vec::new(),
            command_name: command_name.into(),
            required: None,
            command_index: None,
        }
    }

    /// Sets this command to be required
    ///
    ///  - `missing_error_message` is the error message displayed if this command is missing
    pub fn set_required<S: Into<Cow<'static, str>>>(&mut self, missing_error_message: S) {
        self.required = Some(missing_error_message.into());
    }

    /// Sets this command to be not required
    pub fn set_not_required(&mut self) {
        self.required = None;
    }

    /// Adds a new command into the set
    ///
    /// Returns the old command if there is a name conflict
    ///
    ///  - `command` is the name of the command to be matched against
    ///  - `parser` is the parser returned if the command is matched
    pub fn add_command<S: Into<Cow<'static, str>>>(
        &mut self,
        command: S,
        parser: Parser<T, E>,
    ) -> Option<(Cow<'static, str>, Parser<T, E>)> {
        let command = command.into();
        let ret = self.remove(&command);
        self.commands.push((command, parser));
        ret
    }

    /// Parses a command from the argument stream
    ///
    ///  - `argument` is the argument to be parsed
    pub(crate) fn parse(&mut self, argument: OsString) -> Result<(), Error<E>> {
        assert!(self.command_index.is_none());

        let argument = argument
            .into_string()
            .map_err(|string| Error::UnknownCommandOS(string, self.command_name.clone()))?;

        for i in 0..self.commands.len() {
            if self.commands[i].0.as_ref() == argument.as_str() {
                self.command_index = Some(i);
                return Ok(());
            }
        }

        Err(Error::UnknownCommand(argument, self.command_name.clone()))
    }

    /// Verifies if this command is required that it was parsed
    pub(crate) fn finalize(&mut self) -> Result<Option<&mut Parser<T, E>>, Error<E>> {
        let command_index = match self.command_index.take() {
            Some(command_index) => command_index,
            None => match &self.required {
                Some(error_message) => return Err(Error::MissingCommand(error_message.clone())),
                None => return Ok(None),
            },
        };

        Ok(Some(&mut self.commands[command_index].1))
    }

    /// Removes a command based on its name
    ///
    ///  - `command` is the name of the command to be removed
    fn remove(&mut self, command: &str) -> Option<(Cow<'static, str>, Parser<T, E>)> {
        for i in 0..self.commands.len() {
            if command == self.commands[i].0 {
                return Some(self.commands.swap_remove(i));
            }
        }

        None
    }
}

impl<T, E> Deref for Command<T, E> {
    type Target = [(Cow<'static, str>, Parser<T, E>)];

    fn deref(&self) -> &Self::Target {
        &self.commands
    }
}
