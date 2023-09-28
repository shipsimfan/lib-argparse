use crate::{ArgStream, Command, FlagArgument, FlagSet, Positionals, TerminalArgument};
use std::{borrow::Cow, ops::Deref};

pub struct Parser<T, E: 'static = ()> {
    program_name: Option<Cow<'static, str>>,
    description: Option<Cow<'static, str>>,

    flag_arguments: FlagSet<T, E>,
    terminal_argument: TerminalArgument<T, E>,
}

impl<T, E> Parser<T, E> {
    pub fn new() -> Self {
        Parser {
            program_name: None,
            description: None,

            flag_arguments: FlagSet::new(),
            terminal_argument: TerminalArgument::None,
        }
    }

    pub fn program_name(&self) -> Option<&str> {
        self.program_name
            .as_ref()
            .map(|long_name| long_name.deref())
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|long_name| long_name.deref())
    }

    pub fn flag_arguments(&self) -> &[FlagArgument<T, E>] {
        &self.flag_arguments
    }

    pub fn terminal_argument(&self) -> &TerminalArgument<T, E> {
        &self.terminal_argument
    }

    pub fn set_program_name<S: Into<Cow<'static, str>>>(&mut self, program_name: S) {
        self.program_name = Some(program_name.into());
    }

    pub fn set_description<S: Into<Cow<'static, str>>>(&mut self, description: S) {
        self.description = Some(description.into());
    }

    pub fn add_flag_argument(
        &mut self,
        flag_argument: FlagArgument<T, E>,
    ) -> (Option<FlagArgument<T, E>>, Option<FlagArgument<T, E>>) {
        self.flag_arguments.push(flag_argument)
    }

    pub fn set_terminal_argument(&mut self, terminal_argument: TerminalArgument<T, E>) {
        self.terminal_argument = terminal_argument;
    }

    pub fn set_terminal_command(&mut self, command: Command<T, E>) {
        self.terminal_argument = TerminalArgument::Command(command);
    }

    pub fn set_terminal_positionals(&mut self, positionals: Positionals<T, E>) {
        self.terminal_argument = TerminalArgument::Positionals(positionals);
    }

    pub fn clear_terminal_argument(&mut self) {
        self.terminal_argument = TerminalArgument::None;
    }

    pub fn parse(&mut self, options: T) -> Result<T, E> {
        let mut args = ArgStream::new();

        todo!()
    }
}
