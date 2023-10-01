use crate::{Error, Parser};
use std::{borrow::Cow, ffi::OsString, ops::Deref};

/// An unorded set of commands
pub struct Command<T, E: 'static> {
    /// The list of positionals
    ///
    /// The first member of the tuple is the name of the command.
    /// The second member is the help message
    /// The third member is the new parser if the command is reached.
    commands: Vec<(Cow<'static, str>, Cow<'static, str>, Parser<T, E>)>,

    /// The name of the command
    ///
    /// Placed at the "$" in the unknown command error string "unknown $".
    /// It is also placed as the usage hint.
    command_name: Cow<'static, str>,

    /// Determines if this command is required
    ///
    /// The string contained is the error message displayed if this command is missing
    required: Option<Cow<'static, str>>,

    /// The command index and command name found during parsing.
    command_index: Option<(usize, String)>,
}

impl<T, E> Command<T, E> {
    /// Creates a new empty `Command` set
    ///
    ///  - `command_name` is the usage hint. It is also the name of the command placed at the "$" in the unknown error string "unknown $"
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

    /// Sets the command name
    ///
    /// The command name is shown as the usage hint during help.
    /// It is also the name of the command placed at the "$" in the unknown error string "unknown $"
    ///
    ///  - `command_name` is the string the hint will be set to
    pub fn command_name<S: Into<Cow<'static, str>>>(&mut self, command_name: S) {
        self.command_name = command_name.into();
    }

    /// Adds a new command into the set
    ///
    /// Returns the old command if there is a name conflict
    ///
    ///  - `command` is the name of the command to be matched against
    ///  - `help_message` is a description of the command displayed in the help
    ///  - `parser` is the parser returned if the command is matched
    pub fn add_command<S1: Into<Cow<'static, str>>, S2: Into<Cow<'static, str>>>(
        &mut self,
        command: S1,
        help_message: S2,
        parser: Parser<T, E>,
    ) -> Option<(Cow<'static, str>, Parser<T, E>)> {
        let command = command.into();
        let ret = self.remove(&command);
        self.commands.push((command, help_message.into(), parser));
        ret
    }

    /// Generates the help result
    ///
    ///  - `f` is the output
    pub(super) fn generate_usage(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " {}", self.command_name)
    }

    /// Generates the help display
    ///
    ///  - `f` is the output
    pub(super) fn generate_help(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "COMMANDS:")?;

        let mut longest_command = 0;
        for (command, _, _) in &self.commands {
            if command.len() > longest_command {
                longest_command = command.len();
            }
        }
        let command_padding = longest_command + 2;

        for (command, help_message, _) in &self.commands {
            write!(f, "  {}", command)?;
            for _ in 0..command_padding - command.len() {
                write!(f, " ")?;
            }
            writeln!(f, "{}", help_message)?;
        }

        Ok(())
    }

    /// Parses a command from the argument stream
    ///
    ///  - `argument` is the argument to be parsed
    pub(super) fn parse(&mut self, argument: OsString) -> Result<(), Error<E>> {
        assert!(self.command_index.is_none());

        let argument = argument
            .into_string()
            .map_err(|string| Error::UnknownCommandOS(string, self.command_name.clone()))?;

        for i in 0..self.commands.len() {
            if self.commands[i].0.as_ref() == argument.as_str() {
                self.command_index = Some((i, argument));
                return Ok(());
            }
        }

        Err(Error::UnknownCommand(argument, self.command_name.clone()))
    }

    /// Verifies if this command is required that it was parsed
    pub(super) fn finalize(&mut self) -> Result<Option<(&mut Parser<T, E>, String)>, Error<E>> {
        let (command_index, command) = match self.command_index.take() {
            Some(command_index) => command_index,
            None => match &self.required {
                Some(error_message) => return Err(Error::MissingCommand(error_message.clone())),
                None => return Ok(None),
            },
        };

        Ok(Some((&mut self.commands[command_index].2, command)))
    }

    /// Removes a command based on its name
    ///
    ///  - `command` is the name of the command to be removed
    fn remove(&mut self, command: &str) -> Option<(Cow<'static, str>, Parser<T, E>)> {
        for i in 0..self.commands.len() {
            if command == self.commands[i].0 {
                let command = self.commands.swap_remove(i);
                return Some((command.0, command.2));
            }
        }

        None
    }
}

impl<T, E> Deref for Command<T, E> {
    type Target = [(Cow<'static, str>, Cow<'static, str>, Parser<T, E>)];

    fn deref(&self) -> &Self::Target {
        &self.commands
    }
}
