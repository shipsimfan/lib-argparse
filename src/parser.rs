use crate::{ArgStream, Command, FlagArgument, FlagSet, Positionals, TerminalArgument};
use std::{borrow::Cow, ops::Deref};

/// Argument Parser
pub struct Parser<T, E: 'static = ()> {
    /// Program name to display for help
    program_name: Option<Cow<'static, str>>,
    /// Program description for help
    description: Option<Cow<'static, str>>,

    /// Arguments
    flag_arguments: FlagSet<T, E>,
    terminal_argument: TerminalArgument<T, E>,
}

impl<T, E> Parser<T, E> {
    /// Creates a new empty `Parser`
    pub fn new() -> Self {
        Parser {
            program_name: None,
            description: None,

            flag_arguments: FlagSet::new(),
            terminal_argument: TerminalArgument::None,
        }
    }

    /// Returns the program name
    pub fn program_name(&self) -> Option<&str> {
        self.program_name
            .as_ref()
            .map(|long_name| long_name.deref())
    }

    /// Returns the program description
    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|long_name| long_name.deref())
    }

    /// Returns the flag arugments
    pub fn flag_arguments(&self) -> &[FlagArgument<T, E>] {
        &self.flag_arguments
    }

    /// Returns the terminal argument
    pub fn terminal_argument(&self) -> &TerminalArgument<T, E> {
        &self.terminal_argument
    }

    /// Sets the program name to `program_name`
    pub fn set_program_name<S: Into<Cow<'static, str>>>(&mut self, program_name: S) {
        self.program_name = Some(program_name.into());
    }

    /// Sets the program description to `description`
    pub fn set_description<S: Into<Cow<'static, str>>>(&mut self, description: S) {
        self.description = Some(description.into());
    }

    /// Pushes `flag_argument` into the list of flag arguments
    ///
    /// Returns any arguments which have conflicting names
    ///
    /// If there is only one conflict, the conflicting argument will always be the first value of the returned tuple
    pub fn add_flag_argument(
        &mut self,
        flag_argument: FlagArgument<T, E>,
    ) -> (Option<FlagArgument<T, E>>, Option<FlagArgument<T, E>>) {
        self.flag_arguments.push(flag_argument)
    }

    /// Sets the terminal argument to `terminal_argument`
    pub fn set_terminal_argument(&mut self, terminal_argument: TerminalArgument<T, E>) {
        self.terminal_argument = terminal_argument;
    }

    /// Sets the terminal argument to `command`
    pub fn set_terminal_command(&mut self, command: Command<T, E>) {
        self.terminal_argument = TerminalArgument::Command(command);
    }

    /// Sets the terminal arugment to `positionals`
    pub fn set_terminal_positionals(&mut self, positionals: Positionals<T, E>) {
        self.terminal_argument = TerminalArgument::Positionals(positionals);
    }

    /// Removes the terminal argument
    pub fn clear_terminal_argument(&mut self) {
        self.terminal_argument = TerminalArgument::None;
    }

    /// Parses arguments from `argv` into `options`
    pub fn parse(&mut self, options: T) -> Result<T, E> {
        let mut args = ArgStream::new();

        todo!()
    }
}
